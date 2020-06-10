//! Output data structure.

use crate::{
    grid::Grid,
    mcrt::{Light, Material, Settings},
    tree::Cell,
    Set,
};

/// Input data collection.
pub struct Input<'a> {
    /// Simulation settings.
    pub sett: &'a Settings,
    /// Emission source.
    pub light: &'a Light,
    /// Materials.
    pub mats: &'a Set<Material>,
    /// Surface tree.
    pub tree: &'a Cell<'a>,
    /// Regular grid.
    pub grid: &'a Grid,
}

impl<'a> Input<'a> {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(
        sett: &'a Settings,
        light: &'a Light,
        mats: &'a Set<Material>,
        tree: &'a Cell,
        grid: &'a Grid,
    ) -> Self {
        Self {
            sett,
            light,
            mats,
            tree,
            grid,
        }
    }
}
