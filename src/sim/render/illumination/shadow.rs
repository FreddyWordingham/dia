//! Shadowing functions.

use crate::{render::Scene, sample::golden, Dir3, Hit, Ray};
use rand::{rngs::ThreadRng, Rng};

/// Calculate the shadowing factor.
#[inline]
#[must_use]
pub fn shadow(ray: &Ray, scene: &Scene, hit: &Hit, rng: &mut ThreadRng) -> f64 {
    let mut light_ray = Ray::new(*ray.pos(), *hit.side().norm());
    light_ray.travel(scene.sett().bump_dist());
    let light_dir = Dir3::new_normalize(scene.sett().sun_pos() - light_ray.pos());
    *light_ray.dir_mut() = light_dir;

    if let Some(soft_shadows) = scene.sett().soft_shadows() {
        let offset = rng.gen_range(0.0, 2.0);
        let mut total = 0.0;
        for s in 0..soft_shadows {
            let (r, theta) = golden::circle(s, soft_shadows);
            let mut soft_ray = light_ray.clone();
            soft_ray.rotate(r * scene.sett().sun_rad(), theta + offset);
            total += visibility(soft_ray, scene);
        }
        total / soft_shadows as f64
    } else {
        visibility(light_ray.clone(), scene)
    }
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
