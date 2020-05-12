//! Settings implementation.

use crate::clone;
use attr::load;
use std::fmt::{Display, Formatter, Result};

/// MCRT settings structure.
#[load]
pub struct Settings {
    /// Bump distance [m].
    bump_dist: f64,
}

impl Settings {
    clone!(bump_dist, f64);
}

impl Display for Settings {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        write!(fmt, "{:>16} :  {} [m]", "bump_dist", self.bump_dist)
    }
}
