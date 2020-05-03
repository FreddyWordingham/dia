//! Settings implementation.

use crate::clone;
use attr::load;

/// Rendering settings structure.
#[load]
#[derive(Clone)]
pub struct Settings {
    /// Bump distance.
    bump_dist: f64,
}

impl Settings {
    clone!(bump_dist, f64);
}
