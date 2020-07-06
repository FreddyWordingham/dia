//! Scene form implementation.

use crate::{
    display_field, display_field_ln, render,
    render::{Focus, Lens, Sensor},
    AspectRatio, Build, Error, PerlinMap, Pos3, X, Y,
};
use attr::load;
use rand::thread_rng;
use std::{
    fmt::{Display, Formatter},
    path::Path,
};

/// Loadable camera structure.
#[load]
pub struct Scene {
    /// Camera position [m].
    cam_pos: Pos3,
    /// Target position [m].
    cam_tar: Pos3,
    /// Swivel to apply after targeting [deg].
    swivel: [f64; 2],
    /// Perlin noise map segments.
    perlin: [usize; 2],
    /// Sun position when calculating direct shadows [m].
    sun_pos: Pos3,
    /// Sun angular radius when calculating soft shadows [deg].
    sun_rad: f64,
    /// Optional number of ambient occlusion samples.
    ambient_occlusion: Option<i32>,
    /// Optional number of soft shadow samples.
    soft_shadows: Option<i32>,
    /// Horizontal field of view [deg].
    fov: f64,
    /// Aspect ratio.
    aspect_ratio: AspectRatio,
    /// Horizontal resolution.
    res: u64,
    /// Optional super-sampling power.
    ss: Option<i32>,
    /// Optional depth-of-field samples and maximum angular sample [deg].
    dof: Option<(i32, f64)>,
    /// Lighting fractions.
    lighting_fracs: [f64; 3],
    /// Specular lighting power.
    spec_pow: i32,
    /// Shadowing fractions.
    shadowing_fracs: [f64; 2],
    /// Ambient occlusion power.
    ao_pow: i32,
}

impl Build for Scene {
    type Inst = render::Scene;

    #[inline]
    fn build(self, _in_dir: &Path) -> Result<Self::Inst, Error> {
        let dof = if let Some((samples, angle)) = self.dof {
            Some((samples, angle.to_radians()))
        } else {
            None
        };

        let focus = Focus::new(self.cam_pos, self.cam_tar, dof);
        let lens = Lens::new(
            [self.swivel[X].to_radians(), self.swivel[Y].to_radians()],
            self.fov.to_radians(),
        );
        let sensor = Sensor::new(&self.aspect_ratio, self.res, self.ss);

        Ok(Self::Inst::new(
            render::Camera::new(focus, lens, sensor),
            PerlinMap::new(self.perlin, &mut thread_rng()),
            render::Lighting::new(
                self.sun_pos,
                self.sun_rad,
                self.ambient_occlusion,
                self.soft_shadows,
                self.lighting_fracs,
                self.spec_pow,
                self.shadowing_fracs,
                self.ao_pow,
            ),
        ))
    }
}

impl Display for Scene {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        display_field_ln!(fmt, "camera position", &self.cam_pos, "m")?;
        display_field_ln!(fmt, "target position", &self.cam_tar, "m")?;
        display_field_ln!(
            fmt,
            "swivel",
            format!("{} : {}", self.swivel[X], self.swivel[Y]),
            "deg"
        )?;

        display_field!(
            fmt,
            "perlin segments",
            format!("[{}, {}]", self.perlin[X], self.perlin[Y])
        )?;

        display_field_ln!(fmt, "sun position", &self.sun_pos, "m")?;
        display_field_ln!(fmt, "sun angular radius", self.sun_rad, "deg")?;
        if let Some(ambient_occlusion_samples) = self.ambient_occlusion {
            display_field_ln!(fmt, "ambient occlusion samples", ambient_occlusion_samples)?;
        } else {
            display_field_ln!(fmt, "ambient occlusion", "[OFF]")?;
        }
        if let Some(soft_shadows_samples) = self.soft_shadows {
            display_field_ln!(fmt, "soft shadow samples", soft_shadows_samples)?;
        } else {
            display_field_ln!(fmt, "soft shadows", "[OFF]")?;
        };

        display_field_ln!(fmt, "field of view", self.fov, "deg")?;
        display_field_ln!(fmt, "aspect ratio", &self.aspect_ratio)?;
        display_field_ln!(fmt, "horizontal resolution", self.res)?;

        if let Some(ss_power) = self.ss {
            display_field_ln!(fmt, "super sampling power", ss_power)?;
        } else {
            display_field_ln!(fmt, "super sampling", "[OFF]")?;
        }
        if let Some((dof_samples, dof_angle)) = self.dof {
            display_field_ln!(fmt, "depth of field samples", dof_samples)?;
            display_field!(fmt, "depth of field angle", dof_angle, "deg")
        } else {
            display_field!(fmt, "depth of field", "[OFF]")
        }
    }
}
