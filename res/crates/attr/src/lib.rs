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

mod form;
