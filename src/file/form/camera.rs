//! Camera form implementation.

use crate::{
    render::{Camera as CameraInst, Focus, Lens, Sensor},
    AspectRatio, Pos3,
};
use attr::load;

/// Loadable camera structure.
#[load]
pub struct Camera {
    /// Position.
    pos: Pos3,
    /// Target point.
    tar: Pos3,
    /// Horizontal field of view [deg].
    fov: f64,
    /// Aspect ratio.
    aspect_ratio: AspectRatio,
    /// Horizontal resolution.
    res: usize,
}

impl Camera {
    /// Build a camera.
    #[inline]
    #[must_use]
    pub fn build(&self) -> CameraInst {
        let focus = Focus::new(self.pos, self.tar);
        let lens = Lens::new(self.fov.to_radians());
        let sensor = Sensor::new(&self.aspect_ratio, self.res);

        CameraInst::new(focus, lens, sensor)
    }
}
