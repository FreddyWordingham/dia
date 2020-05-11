//! Light module.

use crate::{access, clone, Mesh, Spectrum};

/// Photon emission structure.
pub struct Light {
    /// Surface.
    surf: Mesh,
    /// Emission spectrum.
    spec: Spectrum,
    /// Power [J/s].
    power: f64,
}

impl Light {
    access!(surf, Mesh);
    access!(spec, Spectrum);
    clone!(power, f64);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(surf: Mesh, spec: Spectrum, power: f64) -> Self {
        debug_assert!(power > 0.0);

        Self { surf, spec, power }
    }
}
