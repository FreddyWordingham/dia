//! Light form implementation.

use crate::{access, clone, mcrt::Light as LightInst, Error, Load, Mesh, Redirect, Spectrum};
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

impl Light {
    access!(mesh, String);
    access!(spec, Redirect<Spectrum>);
    clone!(power, f64);

    /// Build a light.
    /// # Errors
    /// if the surface mesh
    /// or the spectrum can not be loaded.
    #[inline]
    pub fn build(&self, in_dir: &Path) -> Result<LightInst, Error> {
        let surf = Mesh::load(&in_dir.join(&self.mesh))?;

        Ok(LightInst::new(surf, self.spec.get(in_dir)?, self.power))
    }
}
