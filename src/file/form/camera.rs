//! Camera form implementation.

use crate::{Camera as CameraInst, Pos3};
use attr::load;

/// Loadable camera structure.
#[load]
pub struct Camera {
    /// Position.
    pos: Pos3,
    /// Target point.
    tar: Pos3,
    /// Image resolution.
    res: (usize, usize),
    /// Horizontal field of view.
    fov: f64,
}

impl Camera {
    /// Build a camera.
    #[inline]
    #[must_use]
    pub fn build(&self) -> CameraInst {
        CameraInst::new(self.pos, self.tar, self.res, self.fov)
    }
}
