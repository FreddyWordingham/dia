//! Settings implementation.

use crate::{access, clone, Pos3};
use attr::load;

/// Rendering settings structure.
#[load]
#[derive(Clone)]
pub struct Settings {
    /// Bump distance.
    bump_dist: f64,
    /// Sun position.
    sun_pos: Pos3,
    /// Specular power.
    spec_pow: i32,
}

impl Settings {
    clone!(bump_dist, f64);
    access!(sun_pos, Pos3);
    clone!(spec_pow, i32);
}
