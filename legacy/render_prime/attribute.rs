//! Attribute structure.

use crate::{clone, report};
use attr::load;
use std::fmt::{Display, Formatter, Result};

/// Rendering attributes structure.
#[load]
pub struct Attribute {
    /// Refractive index.
    ref_index: f64,
}

impl Attribute {
    clone!(ref_index, f64);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(ref_index: f64) -> Self {
        Self { ref_index }
    }
}

impl Display for Attribute {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        write!(
            fmt,
            "{}",
            report::obj("refractive index", self.ref_index).expect("Could not format field.")
        )
    }
}
