//! Output data structure.

use crate::mcrt::Settings;

/// Input data collection.
pub struct Input<'a> {
    /// Simulation settings.
    pub sett: &'a Settings,
}

impl<'a> Input<'a> {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(sett: &'a Settings) -> Self {
        Self { sett }
    }
}
