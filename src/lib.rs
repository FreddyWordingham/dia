//! Library core.

#![warn(
    clippy::all,
    clippy::cargo,
    clippy::missing_docs_in_private_items,
    clippy::nursery,
    clippy::pedantic,
    clippy::restriction
)]
#![allow(
    clippy::as_conversions,
    clippy::implicit_return,
    clippy::integer_arithmetic,
    clippy::integer_division,
    clippy::print_stdout,
    clippy::modulo_arithmetic,
    clippy::unreachable
)]

pub mod error;
pub mod file;
pub mod util;

pub use self::{error::*, file::*, util::*};
