//! Settings implementation.

use crate::clone;
use attr::load;

/// MCRT settings structure.
#[load]
#[derive(Clone)]
pub struct Settings {
    /// Bump distance [m].
    bump_dist: f64,
}

impl Settings {
    clone!(bump_dist, f64);
}
