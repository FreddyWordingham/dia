//! Input module.

use crate::{
    display_field, display_field_ln,
    grid::Grid,
    render::{Attributes, Camera, Settings},
    tree::Cell,
    Mesh, PerlinMap, Set,
};
use palette::{Gradient, LinSrgba};
use std::fmt::{Display, Formatter, Result};

/// Render simulation input structure.
pub struct Input<'a> {
    /// Settings.
    pub sett: &'a Settings,
    /// Surfaces.
    pub surfs: &'a Set<Mesh>,
    /// Colours.
    pub cols: &'a Set<Gradient<LinSrgba>>,
    /// Attributes.
    pub attrs: &'a Set<Attributes>,
    /// Camera.
    pub cam: &'a Camera,
    /// Adaptive tree.
    pub tree: &'a Cell<'a>,
    /// Surface tree.
    pub grid: &'a Grid,
    /// Perlin noise map.
    pub perl: &'a PerlinMap,
}

impl<'a> Input<'a> {
    /// Construct a new instance.
    #[allow(clippy::too_many_arguments)]
    #[inline]
    #[must_use]
    pub const fn new(
        sett: &'a Settings,
        surfs: &'a Set<Mesh>,
        cols: &'a Set<Gradient<LinSrgba>>,
        attrs: &'a Set<Attributes>,
        cam: &'a Camera,
        tree: &'a Cell<'a>,
        grid: &'a Grid,
        perl: &'a PerlinMap,
    ) -> Self {
        Self {
            sett,
            surfs,
            cols,
            attrs,
            cam,
            tree,
            grid,
            perl,
        }
    }
}

impl<'a> Display for Input<'a> {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        display_field_ln!(fmt, "attributes", &self.attrs)?;
        display_field!(fmt, "camera", &self.cam)
    }
}
