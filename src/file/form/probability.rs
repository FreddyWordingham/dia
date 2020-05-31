//! Probability form implementation.

use crate::{Build, Error};
use attr::load;
use std::path::Path;

/// Probability distribution.
#[load]
pub enum Probability {}

impl Build for Probability {
    type Inst = crate::Probability;

    #[inline]
    fn build(self, _in_dir: &Path) -> Result<Self::Inst, Error> {
        Ok(match self {})
    }
}
