//! Photon implementation.

use crate::{access, clone, Ray};

/// Photon structure.
pub struct Photon {
    /// Ray of travel.
    ray: Ray,
    /// Statistical weight.
    weight: f64,
    /// Wavelength [m].
    wavelength: f64,
}

impl Photon {
    access!(ray, ray_mut, Ray);
    clone!(weight, weight_mut, f64);
    clone!(wavelength, wavelength_mut, f64);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(wavelength: f64, ray: Ray) -> Self {
        debug_assert!(wavelength > 0.0);

        Self {
            ray,
            weight: 1.0,
            wavelength,
        }
    }
}