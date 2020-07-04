//! Pixel painter function module.

use crate::{
    render::{Input, Output},
    Ray,
};
use rand::rngs::ThreadRng;
use std::sync::{Arc, Mutex};

/// Pixel painter function type.
pub type Painter = fn(usize, &mut ThreadRng, &Input, &Arc<Mutex<Output>>, f64, [usize; 2], Ray);

pub mod kessler;
pub mod naboo;

pub use self::kessler::*;
pub use self::naboo::*;
