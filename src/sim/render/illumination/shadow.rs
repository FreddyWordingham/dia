//! Shadowing functions.

use crate::{golden, render::Input, Dir3, Hit, Ray};
use rand::{rngs::ThreadRng, Rng};
use std::f64::consts::PI;

const MAX_VISIBILITY_DIST: f64 = 1.0e9;
const AMBIENT_OCCLUSION_POWER: i32 = 8;
const DEFAULT_AMBIENT: f64 = 0.7;

/// Calculate the shadowing factor.
#[inline]
#[must_use]
pub fn shadow(input: &Input, ray: &Ray, hit: &Hit, bump_dist: f64, rng: &mut ThreadRng) -> f64 {
    debug_assert!(bump_dist > 0.0);

    let ambient = if let Some(samples) = input.sett.ambient_occlusion() {
        let offset = rng.gen_range(0.0, 2.0 * PI);
        let mut total = 0.0;
        let mut norm_ray = Ray::new(*ray.pos(), *hit.side().norm());
        norm_ray.travel(bump_dist);
        for n in 0..samples {
            let (phi, theta) = golden::hemisphere(n, samples);
            let mut ambient_ray = norm_ray.clone();
            ambient_ray.rotate(phi, theta + offset);
            total += visibility(input, ambient_ray, bump_dist);
        }
        (total / f64::from(samples)).powi(AMBIENT_OCCLUSION_POWER)
    } else {
        DEFAULT_AMBIENT
    };

    let sun_dir = Dir3::new_normalize(input.sett.sun_pos() - ray.pos());
    let mut light_ray = Ray::new(*ray.pos(), *hit.side().norm());
    light_ray.travel(bump_dist);
    *light_ray.dir_mut() = sun_dir;
    let solar = visibility(input, light_ray, bump_dist);

    ambient + solar
}

/// Calculate the visibility of a given ray.
#[inline]
#[must_use]
pub fn visibility(input: &Input, mut ray: Ray, bump_dist: f64) -> f64 {
    debug_assert!(bump_dist > 0.0);

    let mut vis = 1.0;
    while let Some(hit) = input
        .tree
        .observe(ray.clone(), bump_dist, MAX_VISIBILITY_DIST)
    {
        if vis <= 0.1 {
            break;
        }

        match hit.group() {
            "ground" => {
                // Opaque.
                vis = 0.0;
                break;
            }
            "tree" => {
                // Almost opaque.
                vis *= 0.1;
                ray.travel(hit.dist() + bump_dist);
            }
            _ => {
                // Transparent.
                vis *= 0.75;
                ray.travel(hit.dist() + bump_dist);
            }
        }
    }

    vis
}
