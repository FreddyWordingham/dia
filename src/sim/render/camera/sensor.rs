//! Sensor implementation.

use crate::{clone, render::Focus, AspectRatio};

/// Sensor structure.
pub struct Sensor {
    /// Image resolution.
    res: (usize, usize),
    /// Optional sub-sampling.
    sub_samples: Option<i32>,
}

impl Sensor {
    clone!(res, (usize, usize));
    clone!(sub_samples, Option<i32>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(aspect_ratio: &AspectRatio, hr_res: usize, sub_samples: Option<i32>) -> Self {
        debug_assert!(sub_samples.is_none() || sub_samples.unwrap() > 1);

        Self {
            res: (hr_res, aspect_ratio.vt_res(hr_res)),
        }
    }

    /// Calculate the total number of pixels.
    #[inline]
    #[must_use]
    pub fn num_pixels(&self) -> usize {
        self.res.0 * self.res.1
    }
}
