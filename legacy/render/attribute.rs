//! Attribute implementation.

use crate::clone;
use attr::load;

/// Sensor structure.
#[load]
#[derive(Clone)]
pub struct Attribute {
    /// Transparency.
    transparency: f64,
}

impl Attribute {
    clone!(transparency, f64);
}