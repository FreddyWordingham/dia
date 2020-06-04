//! Mesh form implementation.

use crate::{form::Trans3, report, Build, Error, Load, Transform};
use attr::load;
use std::{
    fmt::{Display, Formatter},
    path::Path,
};

/// Loadable triangle mesh conglomerate structure.
#[load]
pub struct Mesh(
    /// List of object files.
    Vec<String>,
    /// Optional transformation.
    Option<Trans3>,
);

impl Build for Mesh {
    type Inst = crate::Mesh;

    #[inline]
    fn build(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        let trans = if let Some(t) = self.1 {
            Some(t.build(in_dir)?)
        } else {
            None
        };

        let mut tris = Vec::new();
        for name in self.0 {
            let mut obj = Self::Inst::load(&in_dir.join(name))?;
            if let Some(t) = trans {
                obj.transform(&t);
            }
            tris.extend(obj.into_tris())
        }

        Ok(Self::Inst::new(tris))
    }
}
