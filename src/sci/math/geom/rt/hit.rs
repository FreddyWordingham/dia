//! Hit implementation.

use crate::{access, clone, Grp, Side};

/// Hit collision information.
#[derive(Clone)]
pub struct Hit<'a> {
    /// Group hit.
    group: &'a Grp,
    /// Distance to the hit.
    dist: f64,
    /// Normal of the surface.
    side: Side,
}

impl<'a> Hit<'a> {
    clone!(dist, dist_mut, f64);
    access!(side, Side);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(group: &'a Grp, dist: f64, side: Side) -> Self {
        debug_assert!(dist > 0.0);

        Self { group, dist, side }
    }

    /// Access the group str.
    #[inline]
    #[must_use]
    pub fn group(&self) -> &str {
        &self.group
    }
}
