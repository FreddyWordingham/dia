//! Settings implementation.

use crate::{clone, display_field, display_field_ln};
use attr::load;
use std::fmt::{Display, Formatter, Result};

/// Rendering settings structure.
#[load]
pub struct Settings {
    /// Bump distance [m].
    bump_dist: f64,
    /// Number of pixels to simulate in each thread block.
    block_size: u64,
    /// Live rendering setting.
    live: bool,
}

impl Settings {
    clone!(bump_dist, f64);
    clone!(block_size, u64);
    clone!(live, bool);
}

impl Display for Settings {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        display_field_ln!(fmt, "bump distance", self.bump_dist, "m")?;
        display_field_ln!(fmt, "block size", self.block_size)?;
        display_field!(fmt, "live", self.live)
    }
}
