//! Shadowing functions.

use crate::{render::Scene, Dir3, Hit, Ray};

/// Calculate the shadowing factor.
#[inline]
#[must_use]
pub fn shadow(ray: &Ray, scene: &Scene, hit: &Hit) -> f64 {
    let mut light_ray = Ray::new(*ray.pos(), *hit.side().norm());
    light_ray.travel(scene.sett().bump_dist());
    let light_dir = Dir3::new_normalize(scene.sett().sun_pos() - light_ray.pos());
    *light_ray.dir_mut() = light_dir;

    let direct = visibility(light_ray, scene);

    direct
}

/// Calculate the visibility of a given ray.
#[inline]
#[must_use]
fn visibility(mut ray: Ray, scene: &Scene) -> f64 {
    let mut vis = 1.0;
    while let Some(hit) = scene.grid().observe(ray.clone(), scene.sett().bump_dist()) {
        if vis <= 0.0 {
            break;
        }

        match hit.group() {
            _ => {
                vis *= 0.5;
                ray.travel(hit.dist() + scene.sett().bump_dist());
            }
        }
    }

    vis
}
