//! Lens implementation.

use crate::clone;
use crate::display_field;
use std::fmt::{Display, Formatter, Result};

/// Lens structure.
#[derive(Debug)]
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

impl Display for Lens {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        display_field!(fmt, "field of view", self.fov.to_degrees(), "deg")
    }
}
