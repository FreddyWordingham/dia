//! Fogging settings.

use crate::{clone, display_field, display_field_ln};
use std::fmt::{Display, Formatter, Result};

/// Fogging structure.
#[derive(Debug)]
pub struct Fog {
    /// Fog sampling distance.
    dist: f64,
    /// Scaling factor.
    scale: f64,
    /// Power factor.
    power: i32,
}

impl Fog {
    clone!(dist, f64);
    clone!(scale, f64);
    clone!(power, i32);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(dist: f64, scale: f64, power: i32) -> Self {
        debug_assert!(dist > 0.0);
        debug_assert!(scale > 0.0);
        debug_assert!(power > 0);

        Self { dist, scale, power }
    }
}

impl Display for Fog {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        display_field_ln!(fmt, "sampling distance", &self.dist, "m")?;
        display_field_ln!(fmt, "scaling factor", self.scale)?;
        display_field!(fmt, "power factor", self.power)
    }
}
