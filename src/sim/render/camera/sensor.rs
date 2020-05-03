//! Sensor implementation.

use crate::{clone, AspectRatio};

/// Sensor structure.
pub struct Sensor {
    /// Image resolution.
    res: (usize, usize),
    /// Optional sub-sampling power.
    super_sample_power: Option<i32>,
}

impl Sensor {
    clone!(res, (usize, usize));
    clone!(super_sample_power, Option<i32>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(aspect_ratio: &AspectRatio, hr_res: usize, super_sample_power: Option<i32>) -> Self {
        debug_assert!(super_sample_power.is_none() || super_sample_power.unwrap() > 1);

        Self {
            res: (hr_res, aspect_ratio.vt_res(hr_res)),
            super_sample_power,
        }
    }

    /// Calculate the total number of pixels.
    #[inline]
    #[must_use]
    pub const fn num_pixels(&self) -> usize {
        self.res.0 * self.res.1
    }

    /// Calculate the number of sub-samples per pixel.
    #[inline]
    #[must_use]
    pub fn super_samples(&self) -> i32 {
        if let Some(power) = self.super_sample_power {
            power.pow(2)
        } else {
            1
        }
    }
}
