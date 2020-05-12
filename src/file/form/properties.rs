//! Physical properties form implementation.

use crate::{Build, Error, Formula};
use attr::load;
use std::path::Path;

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
        Ok(Self::Inst::new(
            self.ref_index,
            self.scat_coeff,
            self.abs_coeff,
            self.shift_coeff,
            self.asym_fact,
        ))
    }
}
