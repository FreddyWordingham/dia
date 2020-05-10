//! Camera form implementation.

use crate::{access, clone, Redirect, Spectrum};
use attr::load;

/// Loadable light structure.
#[load]
pub struct Light {
    /// Mesh surface.
    mesh: String,
    /// Light power [J/s].
    power: f64,
    /// Light spectrum.
    spec: Redirect<Spectrum>,
}

impl Light {
    access!(mesh, String);
    clone!(power, f64);
    access!(spec, Redirect<Spectrum>);
}
