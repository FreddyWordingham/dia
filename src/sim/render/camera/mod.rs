//! Camera module.

pub mod focus;
pub mod lens;
pub mod sensor;

pub use self::{focus::*, lens::*, sensor::*};

use crate::{access, Pos3, Ray, Rot3};

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
        sub_sample: usize,
        depth_sample: usize,
    ) -> Ray {
        let ray = self.focus.observation_ray(depth_sample);

        let fov = self.lens.fov();
        let delta = fov / (self.sensor.res().0 - 1) as f64;

        let mut theta = (pixel.0 as f64 * delta) - (fov * 0.5);
        let mut phi = (pixel.1 as f64 * delta) - (fov * 0.5);

        // if let Some(sub_samples) = self.sensor.sub_samples() {

        // }

        *ray.dir_mut() = Rot3::from_axis_angle(self.focus.orient().down(), theta)
            * Rot3::from_axis_angle(self.focus.orient().right(), phi)
            * ray.dir();

        ray
    }

    // /// Generate a sub-pixel ray.
    // #[inline]
    // #[must_use]
    // pub fn gen_ss_ray(&self, pos: Point3<f64>, pixel: (usize, usize), sub_sample: usize) -> Ray {
    //     let (xi, yi) = pixel;

    //     let forward = Unit::new_normalize(self.camera.tar() - pos);
    //     let up = Vector3::z_axis();
    //     let right = Unit::new_normalize(forward.cross(&up));

    //     let mut theta = (xi as f64 * self.camera.delta().0) - (self.camera.fov().0 * 0.5);
    //     let mut phi = (yi as f64 * self.camera.delta().1) - (self.camera.fov().1 * 0.5);

    //     let super_samples = self
    //         .quality
    //         .super_samples()
    //         .expect("Bad attempt super sample.");
    //     let sub_delta = self.camera.sub_delta().expect("Bad attempt super sample.");
    //     let sx = (sub_sample % super_samples) as f64 + 0.5;
    //     let sy = (sub_sample / super_samples) as f64 + 0.5;
    //     theta += (sx * sub_delta.0) - (self.camera.delta().0 * 0.5);
    //     phi += (sy * sub_delta.1) - (self.camera.delta().1 * 0.5);

    //     let mut ray = Ray::new(pos, forward);
    //     *ray.dir_mut() = Rotation3::from_axis_angle(&-up, theta)
    //         * Rotation3::from_axis_angle(&right, phi)
    //         * ray.dir();

    //     ray
    // }

    // /// Generate a pixel central ray.
    // #[inline]
    // #[must_use]
    // pub fn gen_pix_ray(&self, pos: Point3<f64>, pixel: (usize, usize)) -> Ray {
    //     let (xi, yi) = pixel;

    //     let forward = Unit::new_normalize(self.camera.tar() - pos);
    //     let up = Vector3::z_axis();
    //     let right = Unit::new_normalize(forward.cross(&up));

    //     let theta = (xi as f64 * self.camera.delta().0) - (self.camera.fov().0 * 0.5);
    //     let phi = (yi as f64 * self.camera.delta().1) - (self.camera.fov().1 * 0.5);

    //     let mut ray = Ray::new(pos, forward);
    //     *ray.dir_mut() = Rotation3::from_axis_angle(&-up, theta)
    //         * Rotation3::from_axis_angle(&right, phi)
    //         * ray.dir();

    //     ray
    // }
}
