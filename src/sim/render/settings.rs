//! Settings implementation.

use crate::{access, clone, Pos3};
use attr::load;

/// Rendering settings structure.
#[load]
#[derive(Clone)]
pub struct Settings {
    /// Bump distance [m].
    bump_dist: f64,
    /// Sun position.
    sun_pos: Pos3,
    /// Sun angular radius [rad].
    sun_rad: f64,
    /// Lighting weights.
    light_weights: [f64; 3],
    /// Specular power.
    spec_pow: i32,
    /// Optional number of soft shadow samples.
    soft_shadows: Option<i32>,
}

impl Settings {
    clone!(bump_dist, f64);
    access!(sun_pos, Pos3);
    clone!(sun_rad, f64);
    access!(light_weights, [f64; 3]);
    clone!(spec_pow, i32);
    clone!(soft_shadows, Option<i32>);
}
