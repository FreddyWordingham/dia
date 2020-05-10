//! Camera form implementation.

use crate::{access, clone, form::Oop};
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
    access!(mesh, String);
    clone!(power, f64);
    access!(spec, Oop<f64>);
}
