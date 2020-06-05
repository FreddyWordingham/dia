//! Shadowing functions.

use crate::{render::Input, Dir3, Hit, Ray};

/// Calculate the shadowing factor.
#[inline]
#[must_use]
pub fn shadow(input: &Input, ray: &Ray, hit: &Hit, bump_dist: f64) -> f64 {
    debug_assert!(bump_dist > 0.0);

    let _sun_dir = Dir3::new_normalize(input.sett.sun_pos() - ray.pos());
    let mut light_ray = Ray::new(*ray.pos(), *hit.side().norm());
    light_ray.travel(bump_dist);
    *light_ray.dir_mut() = Dir3::new_normalize(input.sett.sun_pos() - light_ray.pos());

    let solar = visibility(input, light_ray, bump_dist);
    solar
}

/// Calculate the visibility of a given ray.
#[inline]
#[must_use]
pub fn visibility(input: &Input, mut ray: Ray, bump_dist: f64) -> f64 {
    debug_assert!(bump_dist > 0.0);

    let mut vis = 1.0;
    while let Some(hit) = input.tree.observe(ray.clone(), bump_dist, 1.0e9) {
        if vis <= 0.1 {
            break;
        }

        ray.travel(hit.dist());

        match hit.group() {
            "tree" => {
                // Opaque.
                vis = 0.0;
                break;
            }
            _ => {
                // Transparent.
                vis *= 0.75;
                ray.travel(bump_dist);
            }
        }
    }

    vis
}
