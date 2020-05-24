//! Hit enumeration.

use crate::report;
use std::fmt::{Display, Formatter, Result};

/// Hit determination enumeration.
pub enum Hit {
    /// Voxel boundary collision.
    Voxel(f64),
    /// Scattering event.
    Scattering(f64),
}

impl Hit {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(voxel_dist: f64, scat_dist: f64, bump_dist: f64) -> Self {
        debug_assert!(voxel_dist > 0.0);
        debug_assert!(scat_dist > 0.0);
        debug_assert!(bump_dist > 0.0);

        if voxel_dist <= scat_dist {
            Self::Voxel(voxel_dist)
        } else {
            Self::Scattering(scat_dist)
        }
    }
}

impl Display for Hit {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        let event = format!("Scatter {} [m]", 1.23);
        write!(
            fmt,
            "{}",
            report::obj("event", event).expect("Could not format field.")
        )
    }
}
