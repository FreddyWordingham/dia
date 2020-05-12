//! Light form implementation.

use crate::{Build, Error, Load, Mesh, Redirect, Spectrum};
use attr::load;
use std::fmt::{Display, Formatter};
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
        let spec = self.spec.get(in_dir)?;
        let power = self.power;

        Ok(Self::Inst::new(surf, spec, power))
    }
}

impl Display for Light {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        writeln!(fmt, "{:>16}  > {}", "mesh", self.mesh)?;
        writeln!(fmt, "{:>16}  > {}", "spectrum", self.spec)?;
        write!(fmt, "{:>16}  > {} [J/s]", "power", self.power)
    }
}
