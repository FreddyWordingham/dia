//! Camera module.

pub mod focus;
pub mod lens;
pub mod sensor;

pub use self::{focus::*, lens::*, sensor::*};

use crate::{access, Ray};

/// Camera structure.
pub struct Camera {
    /// Focus.
    focus: Focus,
    /// Lens.
    lens: Lens,
    /// Sensor.
    sensor: Sensor,
}

impl Camera {
    access!(focus, Focus);
    access!(lens, Lens);
    access!(sensor, Sensor);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(focus: Focus, lens: Lens, sensor: Sensor) -> Self {
        Self {
            focus,
            lens,
            sensor,
        }
    }

    /// Generate the nth camera ray.
    #[inline]
    #[must_use]
    pub fn gen_ray(&self, pixel: (usize, usize)) -> Ray {
        debug_assert!(pixel.0 < self.sensor.res().0);
        debug_assert!(pixel.1 < self.sensor.res().1);

         
    }
}
