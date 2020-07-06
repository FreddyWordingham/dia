//! Shadowing functions.

use crate::{
    golden,
    render::{Attributes, Input, Scene},
    Crossing, Dir3, Hit, Ray,
};
use rand::{rngs::ThreadRng, Rng};
use std::f64::consts::PI;

/// Maximum distance tested for ray visibility [m].
const MAX_VISIBILITY_DIST: f64 = 1.0e9;

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

/// Calculate the visibility of a given ray.
#[inline]
#[must_use]
pub fn visibility(input: &Input, mut ray: Ray, bump_dist: f64, mut vis: f64) -> f64 {
    debug_assert!(vis > 0.0);
    debug_assert!(vis <= 1.0);
    debug_assert!(bump_dist > 0.0);

    while let Some(hit) = input
        .tree
        .observe(ray.clone(), bump_dist, MAX_VISIBILITY_DIST)
    {
        if vis < input.sett.min_weight() {
            return 0.0;
        }

        let group = hit.group();
        if let Some(attr) = input.attrs.map().get(group) {
            match attr {
                Attributes::Luminous => {
                    break;
                }
                Attributes::Transparent { abs } => {
                    vis *= 1.0 - *abs;
                    ray.travel(hit.dist() + bump_dist);
                }
                Attributes::Mirror { abs } => {
                    ray.travel(hit.dist());
                    vis *= 1.0 - *abs;
                    *ray.dir_mut() = Crossing::init_ref_dir(
                        ray.dir(),
                        hit.side().norm(),
                        -ray.dir().dot(hit.side().norm()),
                    );
                    ray.travel(bump_dist);
                }
                Attributes::Refractive {
                    abs,
                    inside,
                    outside,
                } => {
                    ray.travel(hit.dist());
                    vis *= 1.0 - abs;

                    let (n_prev, n_curr) = if hit.side().is_inside() {
                        (*inside, *outside)
                    } else {
                        (*outside, *inside)
                    };
                    let crossing = Crossing::new(ray.dir(), hit.side().norm(), n_curr, n_prev);

                    let trans_prob = crossing.trans_prob();
                    let trans_vis = if let Some(trans_dir) = crossing.trans_dir() {
                        let mut trans_ray = ray.clone();
                        *trans_ray.dir_mut() = *trans_dir;
                        trans_ray.travel(bump_dist);

                        visibility(input, trans_ray, bump_dist, vis * trans_prob)
                    } else {
                        0.0
                    };

                    let ref_prob = crossing.ref_prob();
                    let mut ref_ray = ray;
                    *ref_ray.dir_mut() = *crossing.ref_dir();
                    let ref_vis = visibility(input, ref_ray, bump_dist, vis * ref_prob);

                    vis = trans_vis + ref_vis;
                    break;
                }
            }
        } else {
            vis = 0.0;
            break;
        }
    }

    vis
}
