//! Physical properties form implementation.

use crate::{mcrt::DISPLAY_WAVELENGTH, report, Build, Error, Formula};
use attr::load;
use std::{
    fmt::{Display, Formatter},
    path::Path,
};

/// Loadable physical properties structure.
#[load]
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

impl Build for Properties {
    type Inst = crate::mcrt::Properties;

    #[inline]
    fn build(self, _in_dir: &Path) -> Result<Self::Inst, Error> {
        let ref_index = self.ref_index;
        let scat_coeff = self.scat_coeff;
        let abs_coeff = self.abs_coeff;
        let shift_coeff = self.shift_coeff;
        let asym_fact = self.asym_fact;

        Ok(Self::Inst::new(
            ref_index,
            scat_coeff,
            abs_coeff,
            shift_coeff,
            asym_fact,
        ))
    }
}

impl Display for Properties {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        writeln!(
            fmt,
            "{}",
            report::obj_units("(sample wavelength)", DISPLAY_WAVELENGTH * 1e9, "nm")
                .expect("Could not format field.")
        )?;

        writeln!(
            fmt,
            "{}",
            report::obj("refractive index", self.ref_index.y(DISPLAY_WAVELENGTH),)
                .expect("Could not format field.")
        )?;

        writeln!(
            fmt,
            "{}",
            report::obj_units(
                "scattering coefficient",
                self.scat_coeff.y(DISPLAY_WAVELENGTH),
                "m^-1"
            )
            .expect("Could not format field.")
        )?;

        if let Some(abs_coeff) = &self.abs_coeff {
            writeln!(
                fmt,
                "{}",
                report::obj_units("absorption factor", abs_coeff.y(DISPLAY_WAVELENGTH), "m^-1")
                    .expect("Could not format field.")
            )?;
        }

        if let Some(shift_coeff) = &self.shift_coeff {
            writeln!(
                fmt,
                "{}",
                report::obj_units("shifting factor", shift_coeff.y(DISPLAY_WAVELENGTH), "m^-1")
                    .expect("Could not format field.")
            )?;
        }

        write!(
            fmt,
            "{}",
            report::obj("asymmetry factor", self.asym_fact.y(DISPLAY_WAVELENGTH))
                .expect("Could not format field.")
        )
    }
}
