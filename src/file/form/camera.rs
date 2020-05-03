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
    /// Optional depth-of-field samples and maximum angular sample [deg].
    dof: Option<(i32, f64)>,
    /// Optional sub-samples.
    sub_samples: Option<i32>,
}

impl Camera {
    /// Build a camera.
    #[inline]
    #[must_use]
    pub fn build(&self) -> CameraInst {
        let dof = if let Some((samples, angle)) = self.dof {
            Some((samples, angle.to_radians()))
        } else {
            None
        };

        let focus = Focus::new(self.pos, self.tar, dof);
        let lens = Lens::new(self.fov.to_radians());
        let sensor = Sensor::new(&self.aspect_ratio, self.res, self.sub_samples);

        CameraInst::new(focus, lens, sensor)
    }
}
