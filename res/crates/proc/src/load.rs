//! Implementation function of the json related procedural macros.

use crate::proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Item};

/// Implement `Load` trait using json parsing.
#[inline]
#[must_use]
pub fn load_derive_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Item);

    let name = match input {
        Item::Struct(s) => s.ident,
        Item::Enum(e) => e.ident,
        Item::Union(u) => u.ident,
        _ => panic!("Can not derive json for this item."),
    };

    let output = quote! {
        impl crate::Load for #name {
            #[inline]
            fn load(path: &std::path::Path) -> Result<Self, crate::Error> {
                crate::from_json(path)
            }
        }
    };

    TokenStream::from(output)
}
