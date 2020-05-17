//! Settings implementation.

use crate::{clone, report};
use attr::load;
use std::fmt::{Display, Formatter, Result};

/// MCRT settings structure.
#[load]
pub struct Settings {
    /// Number of photons to simulate in each thread block.
    block_size: u64,
    /// Number of photons to simulate.
    num_phot: u64,
    /// Bump distance [m].
    bump_dist: f64,
}

impl Settings {
    clone!(block_size, u64);
    clone!(num_phot, u64);
    clone!(bump_dist, f64);
}

impl Display for Settings {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        writeln!(
            fmt,
            "{}",
            report::obj("block size", self.block_size).expect("Could not format field.")
        )?;
        writeln!(
            fmt,
            "{}",
            report::obj("number of photons", self.num_phot).expect("Could not format field.")
        )?;
        write!(
            fmt,
            "{}",
            report::obj("bump distance", self.bump_dist).expect("Could not format field.")
        )
    }
}
