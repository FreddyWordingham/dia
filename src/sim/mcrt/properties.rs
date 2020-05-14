//! Light module.

use crate::{access, mcrt::Optics, Formula};
use std::fmt::{Display, Formatter};

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
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        write!(fmt, "TODO: PROPERTIES DISPLAY")
    }
}

// impl Display for Settings {
//     #[inline]
//     fn fmt(&self, fmt: &mut Formatter) -> Result {
//         writeln!(
//             fmt,
//             "{}",
//             report::obj("bump distance", self.bump_dist).expect("Could not format field.")
//         )?;
//         write!(
//             fmt,
//             "{}",
//             report::obj("number of photons", self.num_phot).expect("Could not format field.")
//         )
//     }
// }
