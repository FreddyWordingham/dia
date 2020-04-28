//! Orientation implementation.

use crate::{access, Dir3, Pos3, Ray, Vec3};

/// Orientation structure.
pub struct Orient {
    /// Position.
    pos: Pos3,
    /// Forward direction.
    forward: Dir3,
    /// Up direction.
    up: Dir3,
    /// Right direction.
    right: Dir3,
}

impl Orient {
    access!(pos, Pos3);
    access!(forward, Dir3);
    access!(up, Dir3);
    access!(right, Dir3);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(ray: Ray) -> Self {
        let (pos, forward) = ray.destruct();
        let up = Vec3::z_axis();
        let right = Dir3::new_normalize(forward.cross(&up));

        Self {
            pos,
            forward,
            up,
            right,
        }
    }
}
