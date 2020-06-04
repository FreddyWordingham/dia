//! Camera form implementation.

use crate::{
    display_field, display_field_ln, AspectRatio, Build, Error, Pos3,
    {
        render,
        render::{Focus, Lens, Sensor},
    },
};
use attr::load;
use std::{
    fmt::{Display, Formatter},
    path::Path,
};

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
    res: u64,
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

impl Display for Camera {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        display_field_ln!(fmt, "position", &self.pos)?;
        display_field_ln!(fmt, "target", &self.tar, "m")?;
        display_field_ln!(fmt, "field of view", self.fov.to_degrees(), "deg")?;
        display_field_ln!(fmt, "aspect ratio", &self.aspect_ratio)?;
        display_field_ln!(fmt, "horizontal resolution", self.res)?;
        if let Some((dof_samples, dof_angle)) = self.dof {
            display_field_ln!(fmt, "depth of field samples", dof_samples)?;
            display_field_ln!(fmt, "depth of field angle", dof_angle.to_degrees(), "deg")?;
        } else {
            display_field_ln!(fmt, "depth of field", "[OFF]")?;
        }
        if let Some(ss_power) = self.ss {
            display_field!(fmt, "super sampling power", ss_power)
        } else {
            display_field!(fmt, "super sampling", "[OFF]")
        }
    }
}
