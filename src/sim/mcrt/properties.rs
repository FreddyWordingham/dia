//! Light module.

use crate::{access, mcrt::Optics, report, Formula};
use std::fmt::{Display, Formatter, Result};

/// Wavelength [m] to use when printing example values.
const PRINT_WAVELENGTH: f64 = 650e-9;

/// Physical attributes structure.
pub struct Properties {
    /// Refractive index.
    ref_index: Formula,
    /// Scattering coefficient [1/m].
    scat_coeff: Formula,
    /// Absorption coefficient [1/m].
    abs_coeff: Option<Formula>,
    /// Shifting coefficient [1/m].
    shift_coeff: Option<Formula>,
    /// Asymmetry factor.
    asym_fact: Formula,
}

impl Properties {
    access!(ref_index, Formula);
    access!(scat_coeff, Formula);
    access!(abs_coeff, Option<Formula>);
    access!(shift_coeff, Option<Formula>);
    access!(asym_fact, Formula);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(
        ref_index: Formula,
        scat_coeff: Formula,
        abs_coeff: Option<Formula>,
        shift_coeff: Option<Formula>,
        asym_fact: Formula,
    ) -> Self {
        Self {
            ref_index,
            scat_coeff,
            abs_coeff,
            shift_coeff,
            asym_fact,
        }
    }

    /// Generate an optical environment for a given wavelength.
    #[inline]
    #[must_use]
    pub fn env(&self, w: f64) -> Optics {
        let index = self.ref_index.y(w);

        let scat = self.scat_coeff.y(w);

        let abs = if let Some(abs_coeff_formula) = &self.abs_coeff {
            abs_coeff_formula.y(w)
        } else {
            0.0
        };

        let shift = if let Some(shift_coeff_formula) = &self.shift_coeff {
            shift_coeff_formula.y(w)
        } else {
            0.0
        };

        let g = self.asym_fact.y(w);

        Optics::new(index, scat, abs, shift, g)
    }
}

impl Display for Properties {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        let sample = |fmt: &mut Formatter, name: &str, func: &Formula| -> Result {
            writeln!(
                fmt,
                "{}",
                report::obj(name, func.y(PRINT_WAVELENGTH)).expect("Could not format field.")
            )
        };

        writeln!(
            fmt,
            "{}",
            report::obj_units("(sample wavelength)", PRINT_WAVELENGTH * 1e9, "nm")
                .expect("Could not format field.")
        )?;
        sample(fmt, "refractive index", &self.ref_index)?;
        sample(fmt, "scattering coefficient", &self.scat_coeff)?;
        if let Some(abs_coeff) = &self.abs_coeff {
            sample(fmt, "absorption coefficient", abs_coeff)?;
        }
        if let Some(shift_coeff) = &self.shift_coeff {
            sample(fmt, "shifting coefficient", shift_coeff)?;
        }
        sample(fmt, "asymmetry factor", &self.asym_fact)?;

        Ok(())
    }
}
