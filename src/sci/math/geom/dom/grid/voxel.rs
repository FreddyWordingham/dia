//! Regular grid cell scheme.

use crate::{access, clone, Aabb};

/// Regular grid structure.
pub struct Voxel {
    /// Boundary.
    bound: Aabb,
    /// Emissions.
    emissions: f64,
}

impl Voxel {
    access!(bound, Aabb);
    clone!(emissions, f64);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(bound: Aabb) -> Self {
        Self {
            bound,
            emissions: 0.0,
        }
    }
}
