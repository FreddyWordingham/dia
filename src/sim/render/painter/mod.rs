//! Pixel painter function module.

use crate::{
    render::{Input, Output},
    Ray,
};
use rand::rngs::ThreadRng;

/// Pixel painter function type.
pub type Painter = fn(usize, &mut ThreadRng, &Input, &mut Output, f64, [usize; 2], Ray);

pub mod test;

pub use self::test::*;
