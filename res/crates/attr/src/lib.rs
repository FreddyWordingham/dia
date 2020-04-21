//! Support library of attribute macros.

#![warn(
    clippy::all,
    clippy::cargo,
    clippy::missing_docs_in_private_items,
    clippy::nursery,
    clippy::pedantic,
    clippy::restriction
)]
#![allow(clippy::implicit_return, clippy::module_name_repetitions)]

extern crate proc_macro;
extern crate proc_macro2;

mod load;
mod save;

use proc_macro::TokenStream;

/// Create the attribute macro load.
#[proc_macro_attribute]
#[inline]
#[must_use]
pub fn load(metadata: TokenStream, input: TokenStream) -> TokenStream {
    load::implementation(&metadata, input)
}

/// Create the attribute macro save.
#[proc_macro_attribute]
#[inline]
#[must_use]
pub fn save(metadata: TokenStream, input: TokenStream) -> TokenStream {
    save::implementation(&metadata, input)
}
