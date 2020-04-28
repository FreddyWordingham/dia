//! Sensor implementation.

use crate::{clone, AspectRatio};

/// Sensor structure.
pub struct Sensor {
    /// Image resolution.
    res: (usize, usize),
}

impl Sensor {
    clone!(res, (usize, usize));

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(aspect_ratio: &AspectRatio, hr_res: usize) -> Self {
        let res = (hr_res, aspect_ratio.vt_res(hr_res));

        Self { res }
    }

    /// Calculate the total number of pixels.
    #[inline]
    #[must_use]
    pub fn num_pixels(&self) -> usize {
        self.res.0 * self.res.1
    }
}