//! Scene module.

use crate::{
    access, display_field, display_field_ln,
    render::{Camera, Lighting},
};
use std::fmt::{Display, Formatter, Result};

/// Scene setup structure.
#[derive(Debug)]
pub struct Scene {
    /// Camera.
    cam: Camera,
    /// Lighting.
    light: Lighting,
}

impl Scene {
    access!(cam, Camera);
    access!(light, Lighting);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(cam: Camera, light: Lighting) -> Self {
        Self { cam, light }
    }
}

impl Display for Scene {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        display_field_ln!(fmt, "cam", &self.cam)?;
        display_field!(fmt, "lighting", &self.light)
    }
}
