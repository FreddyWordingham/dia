//! Shadowing functions.

use crate::{
    golden,
    render::{naboo::visibility, Input, Scene},
    Dir3, Hit, Ray,
};
use rand::{rngs::ThreadRng, Rng};
use std::f64::consts::PI;

/// Calculate the shadowing factor.
#[inline]
#[must_use]
pub fn shadow(
    input: &Input,
    scene: &Scene,
    ray: &Ray,
    hit: &Hit,
    bump_dist: f64,
    rng: &mut ThreadRng,
) -> f64 {
    debug_assert!(bump_dist > 0.0);

    let sun_dir = Dir3::new_normalize(scene.light().sun_pos() - ray.pos());
    let mut light_ray = Ray::new(*ray.pos(), *hit.side().norm());
    light_ray.travel(bump_dist);
    *light_ray.dir_mut() = sun_dir;

    let solar = if let Some(samples) = scene.light().soft_shadows() {
        let offset = rng.gen_range(0.0, 2.0 * PI);
        let mut total = 0.0;
        for n in 0..samples {
            let (r, theta) = golden::circle(n, samples);
            let mut soft_ray = light_ray.clone();
            soft_ray.rotate(r * scene.light().sun_rad(), theta + offset);
            total += visibility(input, soft_ray, bump_dist, 1.0);
        }
        total / f64::from(samples)
    } else {
        visibility(input, light_ray, bump_dist, 1.0)
    };

    if let Some(samples) = scene.light().ambient_occlusion() {
        let offset = rng.gen_range(0.0, 2.0 * PI);
        let mut total = 0.0;
        let mut norm_ray = Ray::new(*ray.pos(), *hit.side().norm());
        norm_ray.travel(bump_dist);
        for n in 0..samples {
            let (phi, theta) = golden::hemisphere(n, samples);
            let mut ambient_ray = norm_ray.clone();
            ambient_ray.rotate(phi, theta + offset);
            total += visibility(input, ambient_ray, bump_dist, 1.0);
        }
        let ambient = (total / f64::from(samples)).powi(scene.light().ao_pow());

        return ambient.mul_add(
            scene.light().ambient_shadow(),
            solar * scene.light().direct_shadow(),
        );
    };

    solar
}
