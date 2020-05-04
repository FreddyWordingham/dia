//! Camera module.

pub mod focus;
pub mod lens;
pub mod sensor;

pub use self::{focus::*, lens::*, sensor::*};

use crate::{access, Ray, Rot3};

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

    /// Generate a new observation ray.
    #[inline]
    #[must_use]
    pub fn gen_ray(
        &self,
        pixel: (usize, usize),
        offset: f64,
        sub_sample: i32,
        depth_sample: i32,
    ) -> Ray {
        let mut ray = self.focus.observation_ray(offset, depth_sample);

        let fov = self.lens.fov();
        let delta = fov / (self.sensor.res().0 - 1) as f64;

        let mut theta = (pixel.0 as f64 * delta) - (fov * 0.5);
        let mut phi = (pixel.1 as f64 * delta)
            - (fov * 0.5 * (self.sensor.res().1 as f64 / self.sensor.res().0 as f64));

        if let Some(super_sample_power) = self.sensor.super_sample_power() {
            let sub_delta = delta / f64::from(super_sample_power);
            let sx = f64::from(sub_sample % super_sample_power) + 0.5;
            let sy = f64::from(sub_sample / super_sample_power) + 0.5;
            theta += (sub_delta * (0.5 + sx)) - (delta * 0.5);
            phi += (sub_delta * (0.5 + sy)) - (delta * 0.5);
        }

        *ray.dir_mut() = Rot3::from_axis_angle(&self.focus.orient().down(), theta)
            * Rot3::from_axis_angle(self.focus.orient().right(), phi)
            * ray.dir();

        ray
    }
}