//! Settings implementation.

use crate::clone;
use attr::load;
use std::fmt::{Display, Formatter, Result};

/// MCRT settings structure.
#[load]
pub struct Settings {
    /// Bump distance [m].
    bump_dist: f64,
    /// Number of photons to simulate.
    num_phot: u64,
}

impl Settings {
    clone!(bump_dist, f64);
    clone!(num_phot, u64);
}

impl Display for Settings {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        writeln!(fmt, "{:>16} :  {} [m]", "bump dist", self.bump_dist)?;
        write!(fmt, "{:>16} :  {} [m]", "num phot", self.num_phot)
    }
}
