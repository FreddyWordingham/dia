//! Attributes implementation.

use attr::load;
use std::fmt::{Display, Formatter, Result};

/// Rendering attributes.
#[load]
pub enum Attributes {
    /// Mirror.
    Mirror {
        /// Probability of reflection from the surface.
        reflectivity: f64,
    },
    /// Refractive.
    Refractive {
        /// Refractive index.
        index: f64,
    },
}

impl Display for Attributes {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        let kind = match self {
            Self::Mirror { reflectivity } => format!("Mirror: {}", reflectivity),
            Self::Refractive { index } => format!("Refractive: {}", index),
        };
        write!(fmt, "{}", kind)
    }
}
