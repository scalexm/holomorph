use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Fields, Ident, Lit, Path};

pub fn expand_derive(input: DeriveInput) -> proc_macro::TokenStream {
    let name = input.ident;
    let krate = super::get_crate_path(&input.attrs);
    let id = super::parse_value_attribute(&input.attrs, "id")
        .unwrap_or(Lit::Int(syn::parse_str("0").unwrap()));
    let method_body = method_body(&input.data, &name, &krate);

    let (impl_generics, ty_generics) = match input.generics.lifetimes().count() {
        0 => (quote! { <'a> }, quote! {}),
        1 => {
            let lt = input.generics.lifetimes().next().unwrap();
            (quote! { <#lt> }, quote! { <#lt> })
        }
        _ => panic!("can only handle one lifetime parameter"),
    };

    let expanded = quote! {
        impl #impl_generics #krate::Decode #impl_generics for #name #ty_generics {
            const ID: u16 = #id as u16;

            fn decode(src: &mut &'a [u8]) -> Result<Self, #krate::Error> {
                #method_body
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

fn method_body(data: &Data, derive_name: &Ident, krate: &Path) -> TokenStream {
    match data {
        Data::Struct(data) => {
            let fields = match &data.fields {
                Fields::Named(fields) => fields,
                Fields::Unnamed(..) => panic!("cannot handle non named fields"),
                Fields::Unit => return quote! { Ok(Self) },
            };

            let mut flag_offset = 0;
            let decode_fields = fields.named.iter().map(|f| {
                let name = &f.ident;

                // If we have a `#[protocol(flag)]` attribute, start by reading
                // one byte and then extract the correct bit for each field
                // (each consecutive field marked with `#[protocol(flag)]`
                // increases the bit offset to extract).
                if super::has_attribute(&f.attrs, "flag") {
                    let expr = if flag_offset % 8 == 0 {
                        // Start of a new chain of flags: read one byte.
                        quote_spanned! {
                            f.span() =>
                                let flag = u8::decode(src)?;
                                let #name = flag & 1 != 0;
                        }
                    } else {
                        // Mask out the previously read byte to extract our bit.
                        let mask = 1u8 << (flag_offset % 8);
                        quote_spanned! { f.span() => let #name = flag & #mask != 0; }
                    };
                    flag_offset += 1;
                    return expr;
                }

                flag_offset = 0;

                let var_len = super::has_attribute(&f.attrs, "var_length");
                let var_contents = super::has_attribute(&f.attrs, "var_contents");

                // Handle arrays with dynamically-encoded length or dynamicall-encoded
                // content.
                if var_len || var_contents {
                    let decode_len = if var_len {
                        quote! { let len = #krate::Var::<u32>::decode(src)?.0 as usize; }
                    } else {
                        quote! { let len = u16::decode(src)? as usize; }
                    };

                    if var_contents {
                        return quote_spanned! {
                            f.span() =>
                                #decode_len
                                let mut res = Vec::with_capacity(len);
                                for _ in 0..len {
                                    res.push(#krate::Var::decode(src)?.0);
                                }
                                let #name = res.into();
                        };
                    } else {
                        return quote_spanned! {
                            f.span() =>
                                #decode_len
                                let mut res = Vec::with_capacity(len);
                                for _ in 0..len {
                                    res.push(#krate::Decode::decode(src)?);
                                }
                                let #name = res.into();
                        };
                    }
                }

                // Handle arrays with dynamically-encoded length or dynamicall-encoded
                // content.

                // If we have a `#[protocol(var)]` attribute, use the dynamic
                // methods through the use of the `Var` marker type.
                if super::has_attribute(&f.attrs, "var") {
                    quote_spanned! { f.span() => let #name = #krate::Var::decode(src)?.0; }
                } else {
                    quote_spanned! { f.span() => let #name = #krate::Decode::decode(src)?; }
                }
            });

            let enumerate_fields = fields.named.iter().map(|f| {
                let name = &f.ident;
                quote! { #name, }
            });

            quote! {
                #(#decode_fields)*

                Ok(Self {
                    #(#enumerate_fields)*
                })
            }
        }

        Data::Enum(data) => {
            let match_fields = data.variants.iter().map(|v| {
                let name = &v.ident;
                let variant = match &v.fields {
                    Fields::Unnamed(fields) => {
                        &fields
                            .unnamed
                            .iter()
                            .next()
                            .expect("did not expect an empty variant")
                            .ty
                    }
                    _ => panic!("expected a tuple-struct like variant"),
                };
                quote_spanned! {
                    v.span() =>
                        <#variant as #krate::Decode<'_>>::ID => {
                            Ok(#derive_name::#name(<#variant as #krate::Decode<'_>>::decode(src)?))
                        }
                }
            });

            // Start by reading the enum id as an `u16`, then map to the
            // corresponding variant.
            quote! {
                let id = u16::decode(src)?;
                match id {
                    #(#match_fields)*

                    _ => Err(#krate::Error::UnknownVariant(id)),
                }
            }
        }

        Data::Union(..) => panic!("cannot handle unions"),
    }
}
