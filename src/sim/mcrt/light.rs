//! Light structure.

use crate::{access, clone, mcrt::Photon, report, Emit, Mesh, Probability};
use rand::rngs::ThreadRng;
use std::fmt::{Display, Formatter, Result};

/// Photon emission structure.
pub struct Light {
    /// Surface.
    surf: Mesh,
    /// Emission Formula.
    spec: Probability,
    /// Power [J/s].
    power: f64,
}

impl Light {
    access!(surf, Mesh);
    access!(spec, Probability);
    clone!(power, f64);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(surf: Mesh, spec: Probability, power: f64) -> Self {
        debug_assert!(power > 0.0);

        Self { surf, spec, power }
    }

    /// Emit a photon.
    #[inline]
    #[must_use]
    pub fn emit(&self, total_phot: u64, rng: &mut ThreadRng) -> Photon {
        debug_assert!(total_phot > 0);

        let ray = self.surf.cast(rng);
        let wavelength = self.spec.gen(rng);
        let power = self.power / total_phot as f64;

        Photon::new(ray, wavelength, power)
    }
}

impl Display for Light {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        writeln!(
            fmt,
            "{}",
            report::obj("triangles", self.surf.tris().len()).expect("Could not format field.")
        )?;
        writeln!(
            fmt,
            "{}",
            report::obj("Formula", &self.spec).expect("Could not format field.")
        )?;
        write!(
            fmt,
            "{}",
            report::obj_units("power", self.power, "J/s").expect("Could not format field.")
        )
    }
}
