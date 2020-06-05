//! Properties form implementation.

use crate::{display_field, display_field_ln, form, Build, Error};
use attr::load;
use std::{
    fmt::{Display, Formatter},
    path::Path,
};

/// Physical attributes structure.
#[load]
pub struct Properties {
    /// Refractive index.
    ref_index: form::Formula,
    /// Scattering coefficient [1/m].
    scat_coeff: form::Formula,
    /// Absorption coefficient [1/m].
    abs_coeff: Option<form::Formula>,
    /// Shifting coefficient [1/m].
    shift_coeff: Option<form::Formula>,
    /// Asymmetry factor.
    asym_fact: form::Formula,
}

impl Build for Properties {
    type Inst = crate::mcrt::Properties;

    #[inline]
    fn build(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        let ref_index = self.ref_index.build(in_dir)?;
        let scat_coeff = self.scat_coeff.build(in_dir)?;
        let abs_coeff = if let Some(abs_coeff) = self.abs_coeff {
            Some(abs_coeff.build(in_dir)?)
        } else {
            None
        };
        let shift_coeff = if let Some(shift_coeff) = self.shift_coeff {
            Some(shift_coeff.build(in_dir)?)
        } else {
            None
        };
        let asym_fact = self.asym_fact.build(in_dir)?;

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
        display_field_ln!(fmt, "refractive index", &self.ref_index)?;
        display_field_ln!(fmt, "scattering coefficient", &self.scat_coeff)?;
        if let Some(abs_coeff) = &self.abs_coeff {
            display_field_ln!(fmt, "absorption coefficient", abs_coeff)?;
        }
        if let Some(shift_coeff) = &self.shift_coeff {
            display_field_ln!(fmt, "shift coefficient", shift_coeff)?;
        }
        display_field!(fmt, "asymmetry factor", &self.asym_fact)
    }
}
