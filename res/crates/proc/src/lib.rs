//! Support library of procedural macros.

#![warn(
    clippy::all,
    clippy::cargo,
    clippy::missing_docs_in_private_items,
    clippy::nursery,
    clippy::pedantic,
    clippy::restriction
)]
#![allow(clippy::implicit_return, clippy::result_expect_used)]

extern crate proc_macro;
extern crate proc_macro2;

mod hello_macro;

use crate::proc_macro::TokenStream;
use hello_macro::hello_derive_impl;

/// Create the procedural macro `HelloMacro`.
#[proc_macro_derive(HelloMacro)]
#[inline]
#[must_use]
pub fn hello_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("Unable to parse token stream.");
    hello_derive_impl(&ast)
}
