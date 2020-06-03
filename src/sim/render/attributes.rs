//! Attributes implementation.

use crate::report;
use attr::load;
use std::fmt::{Display, Formatter};

/// Rendering attributes.
#[load]
pub enum Attributes {
    /// Mirror.
    Mirror {
        /// Probability of reflection from the surface.
        reflectivity: f64,
    },
}

impl Display for Attributes {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        let name = match self {
            Self::Mirror { .. } => "Mirror",
        };

        write!(
            fmt,
            "{}",
            report::obj("type", name).expect("Could not format name.")
        )
    }
}
