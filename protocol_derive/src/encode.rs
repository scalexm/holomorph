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

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics #krate::Encode for #name #ty_generics #where_clause {
            const ID: u16 = #id as u16;

            fn encode(&self, dst: &mut bytes::BytesMut) {
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
                Fields::Unit => return quote! {},
            };

            let mut flag_offset = 0;
            let encode_fields = fields.named.iter().map(|f| {
                let name = &f.ident;

                // If we have a `#[protocol(flag)]` attribute, init a null byte
                // and then set the correct bit for each field (each consecutive
                // field marked with `#[protocol(flag)]` increases  the bit
                // offset to set).
                if super::has_attribute(&f.attrs, "flag") {
                    // The statement which will be used to set the correct bit.
                    let mask = quote! { flag |= (self.#name as u8) << #flag_offset; };

                    let expr = if flag_offset == 0 {
                        // Start of a new chain of flags: init a null byte and
                        // set the first bit.
                        quote_spanned! {
                            f.span() =>
                                let mut flag: u8 = 0;
                                #mask
                        }
                    } else if flag_offset == 7 {
                        // Last bit of our flag byte: set the correct bit,
                        // encode the whole flag byte and reset the bit offset
                        // to possibly account for another consecutive flag.
                        flag_offset = 0;
                        return quote_spanned! {
                            f.span() =>
                                #mask
                                flag.encode(dst);
                        };
                    } else {
                        // Just set the correct bit.
                        quote_spanned! { f.span() => #mask }
                    };
                    flag_offset += 1;
                    return expr;
                }

                let previous_flag = if flag_offset > 0 {
                    // We just ended a chain of flags: encode the whole flag
                    // byte.
                    quote! { flag.encode(dst); }
                } else {
                    quote! {}
                };

                flag_offset = 0;

                let var_len = super::has_attribute(&f.attrs, "var_length");
                let var_contents = super::has_attribute(&f.attrs, "var_contents");

                // Handle arrays with dynamically-encoded length or dynamicall-encoded
                // content.
                if var_len || var_contents {
                    let encode_len = if var_len {
                        quote! { #krate::Var(self.#name.len() as u32).encode(dst); }
                    } else {
                        quote! { (self.#name.len() as u16).encode(dst); }
                    };

                    if var_contents {
                        return quote_spanned! {
                            f.span() =>
                                #previous_flag
                                #encode_len
                                for v in self.#name.iter() {
                                    #krate::Var(*v).encode(dst);
                                }
                        };
                    } else {
                        return quote_spanned! {
                            f.span() =>
                                #previous_flag
                                #encode_len
                                for v in self.#name.iter() {
                                    v.encode(dst);
                                }
                        };
                    }
                }

                // If we have a `#[protocol(var)]` attribute, use the
                // dynamic methods through the use of the `Var` marker type.
                if super::has_attribute(&f.attrs, "var") {
                    quote_spanned! {
                        f.span() =>
                            #previous_flag
                            #krate::Var(self.#name).encode(dst);
                    }
                } else {
                    quote_spanned! {
                        f.span() =>
                            #previous_flag
                            self.#name.encode(dst);
                    }
                }
            });

            let encode_fields = quote! { #(#encode_fields)* };

            let flag_tail = if flag_offset > 0 {
                // The very last fields were part of a chain of flags: encode
                // this flag byte.
                quote! { flag.encode(dst); }
            } else {
                quote! {}
            };

            quote! {
                #encode_fields
                #flag_tail
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

                // Encode our id then encode ourselves.
                quote_spanned! {
                    v.span() =>
                        #derive_name::#name(v) => {
                            <#variant as #krate::Encode>::ID.encode(dst);
                            v.encode(dst);
                        }
                }
            });

            quote! {
                match self {
                    #(#match_fields)*
                }
            }
        }

        Data::Union(..) => panic!("cannot handle unions"),
    }
}
