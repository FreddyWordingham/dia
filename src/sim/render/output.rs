//! Input module.

use crate::{Error, Save};
use std::{ops::AddAssign, path::Path};

/// Render simulation output structure.
pub struct Output {}

impl Output {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

impl AddAssign<&Self> for Output {
    #[inline]
    fn add_assign(&mut self, _rhs: &Self) {}
}

impl Save for Output {
    #[inline]
    fn save(&self, _out_dir: &Path) -> Result<(), Error> {
        Ok(())
    }
}
