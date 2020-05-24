//! Hit scan implementation.

use crate::Hit;

/// Hit scan result enumeration.
pub enum Scan {
    /// Surface hit.
    Surface {
        /// Hit information.
        hit: Hit,
    },
    /// Cell boundary.
    Boundary {
        /// Distance.
        dist: f64,
    },
    /// Surface and cell boundary collision within the bump distance.
    Both {
        /// Surface hit information.
        hit: Hit,
        /// Boundary distance.
        dist: f64,
    },
}

impl Scan {
    /// Construct a new surface instance.
    #[inline]
    #[must_use]
    pub const fn new_surface(hit: Hit) -> Self {
        Self::Surface { hit }
    }

    /// Construct a new boundary instance.
    #[inline]
    #[must_use]
    pub fn new_boundary(dist: f64) -> Self {
        debug_assert!(dist > 0.0);

        Self::Boundary { dist }
    }

    /// Construct a new both instance.
    #[inline]
    #[must_use]
    pub fn new_both(hit: Hit, dist: f64) -> Self {
        debug_assert!(dist > 0.0);

        Self::Both { hit, dist }
    }
}
