//! Rendering engines.

pub mod naboo;

use crate::{
    render::{Input, Scene},
    Ray,
};
use palette::LinSrgba;
use rand::rngs::ThreadRng;

/// Engine type alias.
pub type Engine = fn(&mut ThreadRng, &Input, &Scene, Ray, f64) -> LinSrgba;
