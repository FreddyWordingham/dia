//! Light module.

use crate::{access, Formula, Range};

/// Physical properties structure.
pub struct Properties {
    /// Scattering coefficient [1/m].
    scat_coeff: Formula,
    /// Absorption coefficient [1/m].
    abs_coeff: Option<Formula>,
    /// Shifting coefficient [1/m].
    shift_coeff: Option<Formula>,
    /// Valid wavelength range.
    range: Range,
}

impl Properties {
    access!(scat_coeff, Formula);
    access!(abs_coeff, Option<Formula>);
    access!(shift_coeff, Option<Formula>);
    access!(range, Range);
}
