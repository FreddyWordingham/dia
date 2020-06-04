//! Settings implementation.

use crate::{clone, display_field_ln, display_field_units, report};
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
        display_field_ln!(fmt, "block size", self.block_size)?;
        display_field_ln!(fmt, "number of photons", self.num_phot)?;
        display_field_units!(fmt, "bump distance", self.bump_dist, "m")
    }
}
