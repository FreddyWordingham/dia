//! Input module.

use crate::{
    access, display_field, display_field_ln,
    render::{Attributes, Camera, Settings},
    Mesh, Set,
};
use palette::{Gradient, LinSrgba};
use std::fmt::{Display, Formatter, Result};

/// Render simulation input structure.
pub struct Input {
    /// Settings.
    sett: Settings,
    /// Surfaces.
    surfs: Set<Mesh>,
    /// Colours.
    cols: Set<Gradient<LinSrgba>>,
    /// Attributes.
    attrs: Set<Attributes>,
    /// Camera.
    cam: Camera,
}

impl Input {
    access!(sett, Settings);
    access!(surfs, Set<Mesh>);
    access!(cols, Set<Gradient<LinSrgba>>);
    access!(attrs, Set<Attributes>);
    access!(cam, Camera);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(
        sett: Settings,
        surfs: Set<Mesh>,
        cols: Set<Gradient<LinSrgba>>,
        attrs: Set<Attributes>,
        cam: Camera,
    ) -> Self {
        Self {
            sett,
            surfs,
            cols,
            attrs,
            cam,
        }
    }
}

impl Display for Input {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        display_field_ln!(fmt, "attributes", &self.attrs)?;
        display_field!(fmt, "camera", &self.cam)
    }
}
