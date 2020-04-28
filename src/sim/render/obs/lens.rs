//! Lens implementation.

use crate::clone;

/// Lens structure.
pub struct Lens {
    /// Field of view.
    fov: f64,
}

impl Lens {
    clone!(fov, f64);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(fov: f64) -> Self {
        debug_assert!(fov > 0.0);

        Self { fov }
    }
}
