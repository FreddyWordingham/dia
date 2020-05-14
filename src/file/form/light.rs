//! Light form implementation.

use crate::{report, Build, Error, Load, Mesh, Redirect, Spectrum};
use attr::load;
use std::{
    fmt::{Display, Formatter},
    path::Path,
};

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
        let spec = self.spec.build(in_dir)?;
        let power = self.power;

        Ok(Self::Inst::new(surf, spec, power))
    }
}

impl Display for Light {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        writeln!(
            fmt,
            "{}",
            report::obj("mesh", &self.mesh).expect("Could not format field.")
        )?;
        writeln!(
            fmt,
            "{}",
            report::obj("spectrum", &self.spec).expect("Could not format field.")
        )?;
        write!(
            fmt,
            "{}",
            report::obj_units("power", self.power, "J/s").expect("Could not format field.")
        )
    }
}
