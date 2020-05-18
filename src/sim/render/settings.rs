//! Settings implementation.

use crate::{clone, report};
use attr::load;
use std::fmt::{Display, Formatter, Result};

/// Rendering settings structure.
#[load]
pub struct Settings {
    /// Bump distance [m].
    bump_dist: f64,
}

impl Settings {
    clone!(bump_dist, f64);
}

impl Display for Settings {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        write!(
            fmt,
            "{}",
            report::obj_units("bump distance", self.bump_dist, "m")
                .expect("Could not format field.")
        )
    }
}
