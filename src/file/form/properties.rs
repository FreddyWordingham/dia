//! Physical properties form implementation.

use crate::{Build, Error, Formula};
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
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        writeln!(fmt, "_some_optical_properties_")
    }
}
