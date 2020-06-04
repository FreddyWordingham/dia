//! Pixel painter function module.

use crate::render::{Input, Output};

/// Pixel painter function type.
pub type Painter = fn(&Input, &mut Output);

pub mod test;

pub use self::test::*;
