//! Axis-aligned-bounding-box implementation.

use crate::{access, Pos3, Vec3};

/// Axis-aligned bounding box geometry.
/// Used for spatial partitioning.
pub struct Aabb {
    /// Minimum bound.
    mins: Pos3,
    /// Maximum bound.
    maxs: Pos3,
}

impl Aabb {
    access!(mins, Pos3);
    access!(maxs, Pos3);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(mins: Pos3, maxs: Pos3) -> Self {
        debug_assert!(mins < maxs);

        Self { mins, maxs }
    }

    /// Construct a new axis-aligned bounding box centred on a given point with given half widths.
    #[inline]
    #[must_use]
    pub fn new_centred(centre: &Pos3, hws: &Vec3) -> Self {
        debug_assert!(hws.iter().all(|x| *x > 0.0));

        Self::new(centre - hws, centre + hws)
    }

    /// Calculate the widths.
    #[inline]
    #[must_use]
    pub fn widths(&self) -> Vec3 {
        self.maxs - self.mins
    }

    /// Calculate the half-widths.
    #[inline]
    #[must_use]
    pub fn half_widths(&self) -> Vec3 {
        self.widths() * 0.5
    }

    /// Calculate the centre position.
    #[inline]
    #[must_use]
    pub fn centre(&self) -> Pos3 {
        nalgebra::center(&self.mins, &self.maxs)
    }

    /// Calculate the surface area.
    #[inline]
    #[must_use]
    pub fn area(&self) -> f64 {
        let ws = self.widths();
        2.0 * ws.z.mul_add(ws.x, ws.x.mul_add(ws.y, ws.y * ws.z))
    }

    /// Calculate the volume.
    #[inline]
    #[must_use]
    pub fn vol(&self) -> f64 {
        let ws = self.widths();
        ws.x * ws.y * ws.z
    }

    /// Determine if the given point if contained.
    #[inline]
    #[must_use]
    pub fn contains(&self, p: &Pos3) -> bool {
        p >= &self.mins && p <= &self.maxs
    }
}
