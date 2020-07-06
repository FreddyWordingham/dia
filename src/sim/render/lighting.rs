//! Lighting settings.

use crate::{access, clone, display_field, display_field_ln, Pos3};
use std::fmt::{Display, Formatter, Result};

/// Lighting structure.
#[derive(Debug)]
pub struct Lighting {
    /// Sun position when calculating direct shadows [m].
    sun_pos: Pos3,
    /// Sun angular radius when calculating soft shadows [deg].
    sun_rad: f64,
    /// Optional number of ambient occlusion samples.
    ambient_occlusion: Option<i32>,
    /// Optional number of soft shadow samples.
    soft_shadows: Option<i32>,
    /// Ambient, diffuse and specular lighting fraction.
    light_fracs: [f64; 3],
    /// Specular lighting power.
    spec_pow: i32,
    /// Ambient and direct shadowing fraction.
    shadow_fracs: [f64; 2],
    /// Ambient occlusion power.
    ao_pow: i32,
}

impl Lighting {
    access!(sun_pos, Pos3);
    clone!(sun_rad, f64);
    clone!(ambient_occlusion, Option<i32>);
    clone!(soft_shadows, Option<i32>);
    access!(light_fracs, [f64; 3]);
    clone!(spec_pow, i32);
    access!(shadow_fracs, [f64; 2]);
    clone!(ao_pow, i32);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(
        sun_pos: Pos3,
        sun_rad: f64,
        ambient_occlusion: Option<i32>,
        soft_shadows: Option<i32>,
        light_fracs: [f64; 3],
        spec_pow: i32,
        shadow_fracs: [f64; 2],
        ao_pow: i32,
    ) -> Self {
        Self {
            sun_pos,
            sun_rad,
            ambient_occlusion,
            soft_shadows,
            light_fracs,
            spec_pow,
            shadow_fracs,
            ao_pow,
        }
    }

    /// Get the ambient lighting fraction.
    #[inline]
    #[must_use]
    pub const fn ambient_light(&self) -> f64 {
        self.light_fracs[0]
    }

    /// Get the diffuse lighting fraction.
    #[inline]
    #[must_use]
    pub const fn diffuse_light(&self) -> f64 {
        self.light_fracs[1]
    }

    /// Get the specular lighting fraction.
    #[inline]
    #[must_use]
    pub const fn specular_light(&self) -> f64 {
        self.light_fracs[2]
    }

    /// Get the ambient shadowing fraction.
    #[inline]
    #[must_use]
    pub const fn ambient_shadow(&self) -> f64 {
        self.shadow_fracs[0]
    }

    /// Get the direct shadowing fraction.
    #[inline]
    #[must_use]
    pub const fn direct_shadow(&self) -> f64 {
        self.shadow_fracs[1]
    }
}

impl Display for Lighting {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        display_field_ln!(fmt, "sun position", &self.sun_pos, "m")?;
        display_field_ln!(fmt, "sun radius", &self.sun_rad.to_degrees(), "deg")?;
        if let Some(ambient_occlusion_samples) = self.ambient_occlusion {
            display_field_ln!(fmt, "ambient occlusion samples", ambient_occlusion_samples)?;
        } else {
            display_field_ln!(fmt, "ambient occlusion", "[OFF]")?;
        }
        if let Some(soft_shadows_samples) = self.soft_shadows {
            display_field_ln!(fmt, "soft shadow samples", soft_shadows_samples)?;
        } else {
            display_field_ln!(fmt, "soft shadows", "[OFF]")?;
        }
        display_field_ln!(
            fmt,
            "lighting fractions",
            &format!(
                "[{} : {} : {}]",
                self.ambient_light(),
                self.diffuse_light(),
                self.specular_light()
            )
        )?;
        display_field_ln!(fmt, "specular power", self.spec_pow)?;
        display_field_ln!(
            fmt,
            "shadowing fractions",
            &format!("[{} : {}]", self.ambient_shadow(), self.direct_shadow())
        )?;
        display_field!(fmt, "ambient occlusion power", self.ao_pow)
    }
}
