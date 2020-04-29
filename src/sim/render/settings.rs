//! Settings implementation.

use crate::clone;
use attr::load;

/// Rendering settings structure.
#[load]
pub struct Settings {
    /// Place holder.
    place_holder: f64,
}

impl Settings {
    clone!(place_holder, f64);
}
