//! Input module.

use crate::{
    display_field, display_field_ln,
    grid::Grid,
    render::{Attributes, Settings},
    tree::Cell,
    Mesh, Set,
};
use palette::{Gradient, LinSrgba};
use std::fmt::{Display, Formatter, Result};

/// Render simulation input structure.
pub struct Input<'a> {
    /// Adaptive tree.
    pub tree: &'a Cell<'a>,
    /// Surface tree.
    pub grid: &'a Grid,
    /// Settings.
    pub sett: &'a Settings,
    /// Surfaces.
    pub surfs: &'a Set<Mesh>,
    /// Colours.
    pub cols: &'a Set<Gradient<LinSrgba>>,
    /// Attributes.
    pub attrs: &'a Set<Attributes>,
}

impl<'a> Input<'a> {
    /// Construct a new instance.
    #[allow(clippy::too_many_arguments)]
    #[inline]
    #[must_use]
    pub const fn new(
        tree: &'a Cell<'a>,
        grid: &'a Grid,
        sett: &'a Settings,
        surfs: &'a Set<Mesh>,
        cols: &'a Set<Gradient<LinSrgba>>,
        attrs: &'a Set<Attributes>,
    ) -> Self {
        Self {
            tree,
            grid,
            sett,
            surfs,
            cols,
            attrs,
        }
    }
}

impl<'a> Display for Input<'a> {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        display_field_ln!(fmt, "tree", &self.tree)?;
        display_field_ln!(fmt, "grid", &self.grid)?;
        display_field_ln!(fmt, "settings", &self.sett)?;
        display_field_ln!(fmt, "surfaces", &self.surfs)?;
        display_field_ln!(fmt, "colours", &format!("total {}", self.cols.map().len()))?;
        display_field!(fmt, "attributes", &self.attrs)
    }
}