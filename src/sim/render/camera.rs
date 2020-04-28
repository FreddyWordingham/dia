//! Camera implementation.

use crate::{access, Dir3, Pos3, Vec3};

/// Camera structure.
pub struct Camera {
    /// Position.
    pos: Pos3,
    /// Target point.
    tar: Pos3,
    /// Forward direction.
    forward: Dir3,
    /// Up direction.
    up: Dir3,
    /// Right direction.
    right: Dir3,
}

impl Camera {
    access!(pos, Pos3);
    access!(tar, Pos3);
    access!(forward, Dir3);
    access!(up, Dir3);
    access!(right, Dir3);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(pos: Pos3, tar: Pos3) -> Self {
        let forward = Dir3::new_normalize(tar - pos);
        let up = Vec3::z_axis();
        let right = Dir3::new_normalize(forward.cross(&up));

        Self {
            pos,
            tar,
            forward,
            right,
            up,
        }
    }
}
