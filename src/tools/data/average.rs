//! Rolling average implementation.

use crate::clone;
use std::ops::AddAssign;

/// Rolling average value recording.
#[derive(Default)]
pub struct Average {
    /// Current average value.
    total: f64,
    /// Total counts so far.
    counts: i32,
}

impl Average {
    clone!(total, f64);
    clone!(counts, i32);

    /// Calculate the average value.
    #[inline]
    #[must_use]
    pub fn ave(&self) -> f64 {
        self.total / self.counts as f64
    }
}

impl AddAssign<f64> for Average {
    #[inline]
    fn add_assign(&mut self, rhs: f64) {
        self.total += rhs;
        self.counts += 1;
    }
}
