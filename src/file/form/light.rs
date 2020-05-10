//! Camera form implementation.

use crate::form::Oop;
use attr::load;

/// Loadable light structure.
#[load]
pub struct Light {
    /// Mesh surface.
    mesh: String,
    /// Light power [J/s].
    power: f64,
    /// Light spectrum.
    spec: Oop<f64>,
}

impl Light {
    // /// Build a light.
    // #[inline]
    // #[must_use]
    // pub fn build(&self) -> LightInst {
    //     LightInst::new()
    // }
}
