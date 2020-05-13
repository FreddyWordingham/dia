//! Painting functions module.

use crate::{render::Scene, Ray};
use palette::LinSrgba;
use rand::rngs::ThreadRng;

pub mod hit;

/// Fragment painter function type.
pub type Painter = fn(usize, &Scene, Ray, &mut ThreadRng) -> LinSrgba;
