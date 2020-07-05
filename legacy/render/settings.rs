//! Settings implementation.

use crate::{access, clone, display_field, display_field_ln, Pos3, X, Y};
use attr::load;
use std::fmt::{Display, Formatter, Result};

/// Rendering settings structure.
#[load]
pub struct Settings {
    /// Number of pixels to simulate in each thread block.
    block_size: u64,
    /// Bump distance [m].
    bump_dist: f64,
    /// Sun position [m].
    sun_pos: Pos3,
    /// Sun radius [rad].
    sun_rad: f64,
    /// Optional number of ambient occlusion samples.
    ambient_occlusion: Option<i32>,
    /// Optional number of soft shadow samples.
    soft_shadows: Option<i32>,
    /// Perlin noise map segments.
    perl_segs: [usize; 2],
    /// Live rendering setting.
    live: bool,
}

impl Settings {
    clone!(block_size, u64);
    clone!(bump_dist, f64);
    clone!(sun_rad, f64);
    access!(sun_pos, Pos3);
    clone!(ambient_occlusion, Option<i32>);
    clone!(soft_shadows, Option<i32>);
    clone!(perl_segs, [usize; 2]);
    clone!(live, bool);
}

impl Display for Settings {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        display_field_ln!(fmt, "block size", self.block_size)?;
        display_field_ln!(fmt, "bump distance", self.bump_dist, "m")?;
        display_field_ln!(fmt, "sun position", self.sun_pos, "m")?;
        display_field_ln!(fmt, "sun radius", self.sun_rad.to_degrees(), "deg")?;
        if let Some(samples) = self.ambient_occlusion {
            display_field_ln!(fmt, "ambient occlusion samples", samples)?;
        } else {
            display_field_ln!(fmt, "ambient occlusion", "OFF")?;
        }
        if let Some(samples) = self.soft_shadows {
            display_field_ln!(fmt, "soft shadow samples", samples)?;
        } else {
            display_field_ln!(fmt, "soft shadows", "OFF")?;
        }
        display_field!(
            fmt,
            "perlin segments",
            format!("[{}, {}]", self.perl_segs[X], self.perl_segs[Y])
        )?;
        display_field!(fmt, "live", self.live)
    }
}
