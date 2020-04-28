//! Camera implementation.

use crate::{access, clone, Dir3, Pos3, Vec3};

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
    /// Image resolution.
    res: (usize, usize),
    /// Scanning delta.
    delta: f64,
}

impl Camera {
    access!(pos, Pos3);
    access!(tar, Pos3);
    access!(forward, Dir3);
    access!(up, Dir3);
    access!(right, Dir3);
    clone!(res, (usize, usize));
    clone!(delta, f64);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(pos: Pos3, tar: Pos3, res: (usize, usize), fov: f64) -> Self {
        debug_assert!(res.0 > 0);
        debug_assert!(res.1 > 0);
        debug_assert!(fov > 0.0);

        let forward = Dir3::new_normalize(tar - pos);
        let up = Vec3::z_axis();
        let right = Dir3::new_normalize(forward.cross(&up));

        let delta = fov / (res.0 - 1) as f64;

        Self {
            pos,
            tar,
            forward,
            right,
            up,
            res,
            delta,
        }
    }

    /// Calculate the number of pixels in the final image.
    #[inline]
    #[must_use]
    pub const fn total_pixels(&self) -> usize {
        self.res.0 * self.res.1
    }
}
