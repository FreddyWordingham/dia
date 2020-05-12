//! Build trait.

use crate::Error;
use std::path::Path;

/// Types implementing this trait can be built at runtime from an input form structure.
pub trait Build {
    type Inst;

    // Build the instance type.
    fn build(self, in_dir: &Path) -> Result<Self::Inst, Error>;
}
