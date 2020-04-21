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
    clippy::float_arithmetic,
    clippy::implicit_return,
    clippy::integer_arithmetic,
    clippy::integer_division,
    clippy::print_stdout,
    clippy::modulo_arithmetic,
    clippy::panic,
    clippy::unreachable
)]

pub mod error;
pub mod file;
pub mod sci;
pub mod tools;
pub mod util;

pub use self::{error::*, file::*, sci::*, tools::*, util::*};
