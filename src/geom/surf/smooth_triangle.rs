//! Smooth triangle implementation.

use crate::{access, Dir3, Pos3, Triangle};

/// Triangle geometry with normal interpolation.
pub struct SmoothTriangle {
    /// Base triangle.
    tri: Triangle,
    /// Normal vectors.
    norms: [Dir3; 3],
}

impl SmoothTriangle {
    access!(tri, Triangle);
    access!(norms, [Dir3; 3]);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(tri: Triangle, norms: [Dir3; 3]) -> Self {
        debug_assert!(norms.iter().all(|&n| n.dot(tri.plane_norm()) > 0.0));

        Self { tri, norms }
    }

    /// Construct a new instance from vertices.
    #[inline]
    #[must_use]
    pub fn new_from_verts(verts: [Pos3; 3], norms: [Dir3; 3]) -> Self {
        Self::new(Triangle::new(verts), norms)
    }
}
