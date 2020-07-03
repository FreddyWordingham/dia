//! Pixel painter function module.

use crate::{
    render::{Input, Output},
    Ray,
};
use rand::rngs::ThreadRng;

/// Pixel painter function type.
pub type Painter = fn(usize, &mut ThreadRng, &Input, &mut Output, f64, [usize; 2], Ray);

pub mod kessler;
pub mod naboo;

pub use self::kessler::*;
pub use self::naboo::*;
