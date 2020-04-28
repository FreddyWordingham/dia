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
    pub fn new(ray: Ray) -> Self {
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

    /// Set the orientation's forward direction and update the right direction.
    #[inline]
    pub fn set_forward(&mut self, dir: Dir3) {
        self.forward = dir;
        self.right = Dir3::new_normalize(self.forward.cross(&self.up));
    }

    /// Set the orientation's up direction and update the right direction.
    #[inline]
    pub fn set_up(&mut self, dir: Dir3) {
        self.up = dir;
        self.right = Dir3::new_normalize(self.forward.cross(&self.up));
    }
}
