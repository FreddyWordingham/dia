//! Mesh form implementation.

use crate::{report, Build, Error, Load};
use attr::load;
use std::{
    fmt::{Display, Formatter},
    path::Path,
};

/// Loadable triangle mesh conglomerate structure.
#[load]
pub struct Mesh(
    /// List of object files
    Vec<String>,
);

impl Build for Mesh {
    type Inst = crate::Mesh;

    #[inline]
    fn build(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        let mut tris = Vec::new();
        for name in self.0 {
            let obj = Self::Inst::load(&in_dir.join(name))?;
            tris.extend(obj.into_tris())
        }

        Ok(Self::Inst::new(tris))
    }
}

impl Display for Mesh {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        write!(
            fmt,
            "{}",
            report::list("base objs", &self.0).expect("Could not format field.")
        )
    }
}
