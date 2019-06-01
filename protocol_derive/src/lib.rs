//! Procedural macro enabling auto-derive implementations of the `Encode` and
//! `Decode` traits from the `protocol` crate. Example:
//!
//! ```rust
//! use protocol_derive::{Encode, Decode};
//! use std::borrow::Cow;
//!
//! #[derive(Clone, Encode, Decode)]
//! #[protocol(crate = "protocol", id = 47)]
//! struct HelloMessage<'a> {
//!     id: u32,
//!     #[protocol(var)] key: i64,
//!     #[protocol(flag)] has_foo: bool,
//!     #[protocol(flag)] has_bar: bool,
//!     name: &'a str,
//!     #[protocol(var_length)] actors: Cow<'a, [Actor<'a>]>,
//! }
//!
//! #[derive(Copy, Clone, Encode, Decode)]
//! #[protocol(crate = "protocol", id = 52)]
//! struct Actor<'a> {
//!     level: i32,
//!     name: &'a str,
//!     #[protocol(var)] emotes: &'a [u8],
//! }
//! ```
//!
//! Usable attributes:
//! * `#[protocol(crate = "protocol")]`: give a path where to find the `protocol`
//!    crate; will default to `crate` if absent
//! * `#[protocol(id = 47)]`: generate a `Encode::id` / `Decode::id` method
//!    implementation returning `47`
//! * `#[protocol(var)]`: dynamically encode an integer type on a variable number
//!    of bytes *OR* dynamically encode the length of an `&'_ [u8]` on a variable
//!    number of bytes
//! * `#[protocol(flag)]`: treat a `bool` as a bit offset of an `u8`; consecutive
//!   `bool` fields marked with `#[protocol(flag)]` will use the same `u8` (up to
//!   `8` consecutive fields of course)
//! * `#[protocol(var_length)]`: dynamically encode the length of an array of
//!    arbitrary types on a variable number of bytes
//! * `#[protocol(var_content)]`: dynamically encode each value of an array of
//!   integer types on a variable number of bytes; this attribute can be combined
//!   with the previous one so that both the length and the contents are
//!   dynamically encoded

#![deny(rust_2018_idioms)]

extern crate proc_macro;

mod decode;
mod encode;

use syn::{parse_macro_input, Attribute, DeriveInput, Lit, Meta, NestedMeta, Path};

#[proc_macro_derive(Decode, attributes(protocol))]
/// Generate an auto-derive implementation for the `Decode` trait.
pub fn derive_decode(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    decode::expand_derive(parse_macro_input!(input as DeriveInput))
}

#[proc_macro_derive(Encode, attributes(protocol))]
/// Generate an auto-derive implementation for the `Encode` trait.
pub fn derive_encode(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    encode::expand_derive(parse_macro_input!(input as DeriveInput))
}

fn get_crate_path<'a>(attrs: impl IntoIterator<Item = &'a Attribute>) -> Path {
    match parse_value_attribute(attrs, "crate") {
        Some(Lit::Str(path)) => syn::parse_str(&path.value()).expect("could not parse path"),

        _ => syn::parse_str("crate").expect("could not parse path"),
    }
}

/// Return `val` in `#[protocol(name = val)]`.
fn parse_value_attribute<'a>(
    attrs: impl IntoIterator<Item = &'a Attribute>,
    name: &str,
) -> Option<Lit> {
    for attr in attrs {
        let meta = attr.parse_meta().expect("cannot parse attribute");
        if meta.name() != "protocol" {
            continue;
        }

        let list = match meta {
            Meta::List(list) => list,
            _ => continue,
        };

        for nested in list.nested.iter() {
            let meta = match nested {
                NestedMeta::Meta(meta) => meta,
                _ => continue,
            };

            if meta.name() != name {
                continue;
            }

            match meta {
                Meta::NameValue(nv) => return Some(nv.lit.clone()),
                _ => panic!("expected a name-value attribute"),
            };
        }
    }
    None
}

/// Check whether the given set of attrs contains `#[protocol(name)]`.
fn has_attribute<'a>(attrs: impl IntoIterator<Item = &'a Attribute>, name: &str) -> bool {
    for attr in attrs {
        let meta = attr.parse_meta().expect("cannot parse attribute");
        if meta.name() != "protocol" {
            continue;
        }

        let list = match meta {
            Meta::List(list) => list,
            _ => continue,
        };

        for nested in list.nested.iter() {
            let meta = match nested {
                NestedMeta::Meta(meta) => meta,
                _ => continue,
            };

            if meta.name() == name {
                return true;
            }
        }
    }
    false
}
