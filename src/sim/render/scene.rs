//! Scene module.

use crate::{
    access, display_field, display_field_ln,
    render::{Camera, Lighting},
    PerlinMap,
};
use std::fmt::{Display, Formatter, Result};

/// Scene setup structure.
#[derive(Debug)]
pub struct Scene {
    /// Camera.
    cam: Camera,
    /// Two-dimensional Perlin noise map.
    perlin: PerlinMap,
    /// Lighting.
    light: Lighting,
}

impl Scene {
    access!(cam, Camera);
    access!(perlin, PerlinMap);
    access!(light, Lighting);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(cam: Camera, perlin: PerlinMap, light: Lighting) -> Self {
        Self { cam, perlin, light }
    }
}

impl Display for Scene {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        display_field_ln!(fmt, "cam", &self.cam)?;
        display_field_ln!(fmt, "Perlin noise map", &self.perlin)?;
        display_field!(fmt, "lighting", &self.light)
    }
}