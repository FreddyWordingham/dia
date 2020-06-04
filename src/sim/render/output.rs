//! Input module.

use crate::{Error, Save};
use std::path::Path;

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

impl Save for Output {
    fn save(&self, _out_dir: &Path) -> Result<(), Error> {
        Ok(())
    }
}
