//! Light structure.

use crate::{access, clone, report, Mesh, Spectrum};
use std::fmt::{Display, Formatter, Result};

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
            report::obj("spectrum", &self.spec).expect("Could not format field.")
        )?;
        write!(
            fmt,
            "{}",
            report::obj_units("power", self.power, "J/s").expect("Could not format field.")
        )
    }
}
