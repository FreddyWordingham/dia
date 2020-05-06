//! Shadowing functions.

use crate::{render::Scene, sample::golden, Crossing, Dir3, Hit, Ray};
use rand::{rngs::ThreadRng, Rng};

/// Calculate the shadowing factor.
#[inline]
#[must_use]
pub fn shadow(ray: &Ray, scene: &Scene, hit: &Hit, rng: &mut ThreadRng) -> f64 {
    let mut light_ray = Ray::new(*ray.pos(), *hit.side().norm());
    light_ray.travel(scene.sett().bump_dist());
    let light_dir = Dir3::new_normalize(scene.sett().sun_pos() - light_ray.pos());
    *light_ray.dir_mut() = light_dir;

    let mut ambient = if let Some(ambient_occlusion) = scene.sett().ambient_occlusion() {
        let offset = rng.gen_range(0.0, 2.0);
        let mut total = 0.0;
        let mut norm_ray = Ray::new(*ray.pos(), *hit.side().norm());
        norm_ray.travel(scene.sett().bump_dist());
        for a in 0..ambient_occlusion {
            let (phi, theta) = golden::hemisphere(a, ambient_occlusion);
            let mut ambi_ray = norm_ray.clone();
            ambi_ray.rotate(phi, theta + offset);
            total += visibility(ambi_ray, scene);
        }
        (total / f64::from(ambient_occlusion)).powi(scene.sett().ambient_occlusion_power())
    } else {
        1.0
    };

    let mut solar = if let Some(soft_shadows) = scene.sett().soft_shadows() {
        let offset = rng.gen_range(0.0, 2.0);
        let mut total = 0.0;
        for s in 0..soft_shadows {
            let (r, theta) = golden::circle(s, soft_shadows);
            let mut soft_ray = light_ray.clone();
            soft_ray.rotate(r * scene.sett().sun_rad(), theta + offset);
            total += visibility(soft_ray, scene);
        }
        total / f64::from(soft_shadows)
    } else {
        visibility(light_ray, scene)
    };

    ambient *= scene.sett().shadow_weights()[0];
    solar *= scene.sett().shadow_weights()[1];

    ambient + solar
}

/// Calculate the visibility of a given ray.
#[inline]
#[must_use]
pub fn visibility(mut ray: Ray, scene: &Scene) -> f64 {
    let mut vis = 1.0;
    while let Some(hit) = scene.grid().observe(ray.clone(), scene.sett().bump_dist()) {
        if vis <= 0.1 {
            break;
        }

        ray.travel(hit.dist());

        match hit.group() {
            1 => {
                // Reflective.
                vis *= 0.9;
                *ray.dir_mut() = Crossing::init_ref_dir(
                    ray.dir(),
                    hit.side().norm(),
                    -ray.dir().dot(hit.side().norm()),
                );
                ray.travel(scene.sett().bump_dist());
            }
            2 | 11..=13 => {
                // Refractive.
                vis *= 0.9;

                {
                    let (n0, n1) = if hit.side().is_inside() {
                        (crate::render::paint::hit::REF_INDEX, 1.0)
                    } else {
                        (1.0, crate::render::paint::hit::REF_INDEX)
                    };
                    let crossing = Crossing::new(ray.dir(), hit.side().norm(), n0, n1);
                    if let Some(trans_dir) = crossing.trans_dir() {
                        // let ref_prob = crossing.ref_prob();
                        // let trans_prob = 1.0 - ref_prob;

                        // let mut ref_ray = Ray::new(
                        //     *ray.pos(),
                        //     Crossing::init_ref_dir(
                        //         ray.dir(),
                        //         hit.side().norm(),
                        //         -ray.dir().dot(hit.side().norm()),
                        //     ),
                        // );
                        // ref_ray.travel(scene.sett().bump_dist());
                        // let ref_vis = visibility(ref_ray, scene);

                        // let mut trans_ray = ray;
                        // *trans_ray.dir_mut() = *trans_dir;
                        // trans_ray.travel(scene.sett().bump_dist());
                        // let trans_vis = visibility(trans_ray, scene);

                        // return vis * ref_vis.mul_add(ref_prob, trans_vis * trans_prob);
                        *ray.dir_mut() = *trans_dir;
                        ray.travel(scene.sett().bump_dist());
                    } else {
                        *ray.dir_mut() = *crossing.ref_dir();
                        ray.travel(scene.sett().bump_dist());
                    }
                }
            }
            3..=5 => {
                // Transparent.
                vis *= 0.9;
                ray.travel(scene.sett().bump_dist());
            }
            14 | 18 => {
                // Translucent.
                vis *= 0.5;
                ray.travel(scene.sett().bump_dist());
            }
            31 => {
                break;
            }
            _ => {
                // Opaque.
                vis = 0.0;
                break;
            }
        }
    }

    vis
}
