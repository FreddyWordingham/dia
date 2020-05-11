//! Light form implementation.

use crate::{access, clone, Redirect, Spectrum};
use attr::load;

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

    // /// Build a light.
    // #[inline]
    // #[must_use]
    // pub fn build(&self) -> LightInst {
    //     LightInst::new(surf, spec, power)
    // }
}
