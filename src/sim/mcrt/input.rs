//! Output data structure.

use crate::{
    grid::Grid,
    mcrt::{Light, Properties, Settings},
    tree::Cell,
    Set,
};

/// Input data collection.
pub struct Input<'a> {
    /// Simulation settings.
    pub sett: &'a Settings,
    /// Properties.
    pub props: &'a Set<Properties>,
    /// Emission source.
    pub light: &'a Light,
    /// Regular grid.
    pub grid: &'a Grid,
    /// Surface tree.
    pub tree: &'a Cell<'a>,
}

impl<'a> Input<'a> {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(
        sett: &'a Settings,
        props: &'a Set<Properties>,
        light: &'a Light,
        grid: &'a Grid,
        tree: &'a Cell,
    ) -> Self {
        Self {
            sett,
            props,
            light,
            grid,
            tree,
        }
    }
}
