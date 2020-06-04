//! Camera form implementation.

use crate::{
    AspectRatio, Build, Error, Pos3,
    {
        render,
        render::{Focus, Lens, Sensor},
    },
};
use attr::load;
use std::path::Path;

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
    /// Optional super-sampling power.
    ss: Option<i32>,
}

impl Build for Camera {
    type Inst = render::Camera;

    #[inline]
    fn build(self, _in_dir: &Path) -> Result<Self::Inst, Error> {
        let dof = if let Some((samples, angle)) = self.dof {
            Some((samples, angle.to_radians()))
        } else {
            None
        };

        let focus = Focus::new(self.pos, self.tar, dof);
        let lens = Lens::new(self.fov.to_radians());
        let sensor = Sensor::new(&self.aspect_ratio, self.res, self.ss);

        Ok(Self::Inst::new(focus, lens, sensor))
    }
}
