//! Settings implementation.

use crate::{access, clone, display_field, display_field_ln, Group};
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
    /// Initial material.
    init_mat: Group,
}

impl Settings {
    clone!(block_size, u64);
    clone!(num_phot, u64);
    clone!(bump_dist, f64);
    access!(init_mat, Group);
}

impl Display for Settings {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        display_field_ln!(fmt, "block size", self.block_size)?;
        display_field_ln!(fmt, "number of photons", self.num_phot)?;
        display_field_ln!(fmt, "bump distance", self.bump_dist, "m")?;
        display_field!(fmt, "initial material", &self.init_mat)
    }
}
