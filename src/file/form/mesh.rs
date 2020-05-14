//! Mesh form implementation.

use crate::{Build, Error};
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
        let tris = Vec::new();
        for name in self.0 {
            // let surf = Mesh::load(&in_dir.join(&self.mesh))?;
        }

        Ok(Self::Inst::new(tris))
    }
}

impl Display for Mesh {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        // let s = values(&self.0).expect("Could not ");
        // write!(fmt, "{:>16} :  {}", "objs", s)
        write!(fmt, "")
    }
}
