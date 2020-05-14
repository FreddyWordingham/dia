//! Settings implementation.

use crate::{clone, report};
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
        writeln!(
            fmt,
            "{}",
            report::obj("bump distance", self.bump_dist).expect("Could not format field.")
        )?;
        write!(
            fmt,
            "{}",
            report::obj("number of photons", self.num_phot).expect("Could not format field.")
        )
    }
}
