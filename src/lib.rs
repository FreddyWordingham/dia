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
    clippy::else_if_without_else,
    clippy::float_arithmetic,
    clippy::implicit_return,
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::integer_division,
    clippy::modulo_arithmetic,
    clippy::option_unwrap_used,
    clippy::panic,
    clippy::print_stdout,
    clippy::unreachable
)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
)]

pub mod error;
pub mod file;
pub mod sci;
pub mod sim;
pub mod tools;
pub mod util;

pub use self::{error::*, file::*, sci::*, sim::*, tools::*, util::*};
