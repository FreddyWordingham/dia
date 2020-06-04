//! Light form implementation.

use crate::{display_field, display_field_ln, form, Build, Error, Redirect};
use attr::load;
use std::{
    fmt::{Display, Formatter},
    path::Path,
};

/// Loadable light structure.
#[load]
pub struct Light {
    /// Mesh surface.
    surf: form::Mesh,
    /// Light spectrum.
    spec: Redirect<form::Probability>,
    /// Light power [J/s].
    power: f64,
}

impl Build for Light {
    type Inst = crate::mcrt::Light;

    #[inline]
    fn build(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        let surf = self.surf.build(in_dir)?;
        let spec = self.spec.build(in_dir)?.build(in_dir)?;
        let power = self.power;

        Ok(Self::Inst::new(surf, spec, power))
    }
}

impl Display for Light {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        display_field_ln!(fmt, "surf", &self.surf)?;
        display_field_ln!(fmt, "spec", &self.spec)?;
        display_field!(fmt, "power", self.power, "J/s")
    }
}
