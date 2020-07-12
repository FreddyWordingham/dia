//! Settings implementation.

use crate::{clone, display_field, display_field_ln, render::Order};
use attr::load;
use std::fmt::{Display, Formatter, Result};

/// Rendering settings structure.
#[load]
pub struct Settings {
    /// Bump distance [m].
    bump_dist: f64,
    /// Number of pixels to simulate in update block.
    block_size: u64,
    /// Number of pixels to simulate in each thread block.
    sub_block_size: u64,
    /// Minimum photon weight.
    min_weight: f64,
    /// Live rendering setting.
    live: bool,
    /// Rendering order.
    order: Order,
}

impl Settings {
    clone!(bump_dist, f64);
    clone!(block_size, u64);
    clone!(sub_block_size, u64);
    clone!(min_weight, f64);
    clone!(live, bool);
    clone!(order, Order);
}

impl Display for Settings {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        display_field_ln!(fmt, "bump distance", self.bump_dist, "m")?;
        display_field_ln!(fmt, "block size", self.block_size)?;
        display_field_ln!(fmt, "sub block size", self.block_size)?;
        display_field_ln!(fmt, "minimum photon weight", self.min_weight)?;
        display_field_ln!(fmt, "live", self.live)?;
        display_field!(fmt, "rendering order", &self.order)
    }
}
