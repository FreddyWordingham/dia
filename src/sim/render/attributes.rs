//! Attributes implementation.

use attr::load;
use std::fmt::{Display, Formatter, Result};

/// Rendering attributes.
#[load]
pub enum Attributes {
    /// Mirror.
    Mirror {
        /// Absorption fraction.
        abs: f64,
    },
    /// Refractive.
    Refractive {
        /// Absorption fraction.
        abs: f64,
        /// Internal refractive index.
        inside: f64,
        /// Extental refractive index.
        outside: f64,
    },
}

impl Display for Attributes {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        let kind = match self {
            Self::Mirror { abs } => format!("Mirror: [{}]", abs),
            Self::Refractive {
                abs,
                inside,
                outside,
            } => format!("Refractive: [{}]\t\t{}:|{}", abs, inside, outside),
        };
        write!(fmt, "{}", kind)
    }
}
