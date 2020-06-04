//! Settings implementation.

use crate::{clone, display_field, display_field_ln, report};
use attr::load;
use std::fmt::{Display, Formatter, Result};

/// Rendering settings structure.
#[load]
pub struct Settings {
    /// Number of pixels to simulate in each thread block.
    block_size: u64,
    /// Bump distance [m].
    bump_dist: f64,
}

impl Settings {
    clone!(block_size, u64);
    clone!(bump_dist, f64);
}

impl Display for Settings {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        display_field_ln!(fmt, "block size", self.block_size)?;
        display_field!(fmt, "bump distance", self.bump_dist)
    }
}
