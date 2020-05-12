//! Light form implementation.

use crate::{Build, Error, Load, Mesh, Redirect, Spectrum};
use attr::load;
use std::path::Path;

/// Loadable light structure.
#[load]
pub struct Light {
    /// Mesh surface.
    mesh: String,
    /// Light spectrum.
    spec: Redirect<Spectrum>,
    /// Light power [J/s].
    power: f64,
}

impl Build for Light {
    type Inst = crate::mcrt::Light;

    #[inline]
    fn build(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        let surf = Mesh::load(&in_dir.join(&self.mesh))?;

        Ok(Self::Inst::new(surf, self.spec.get(in_dir)?, self.power))
    }
}
