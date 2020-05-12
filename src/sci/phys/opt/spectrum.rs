//! Spectrum implementation.

use attr::load;
use rand::rngs::ThreadRng;
use std::fmt::{Display, Formatter, Result};

/// Spectrum enumeration implementation.
#[load]
#[derive(Clone)]
pub enum Spectrum {
    /// Single wavelength.
    Laser(f64),
}

impl Spectrum {
    /// Construct a new laser spectrum.
    #[inline]
    #[must_use]
    pub fn new_laser(wavelength: f64) -> Self {
        debug_assert!(wavelength > 0.0);

        Self::Laser { 0: wavelength }
    }

    /// Sample the spectrum for a wavelength.
    #[inline]
    #[must_use]
    pub fn sample(&self, _rng: &mut ThreadRng) -> f64 {
        match self {
            Self::Laser(w) => *w,
        }
    }
}

impl Display for Spectrum {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        match self {
            Self::Laser(w) => write!(fmt, "laser {}nm", w * 1e9),
        }
    }
}
