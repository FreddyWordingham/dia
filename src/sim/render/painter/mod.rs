//! Pixel painter functions.

use crate::{
    render::{illumination, Attributes, Event, Input, Scene},
    Crossing, Dir3, Hit, Ray, Trace,
};
use palette::LinSrgba;
use rand::rngs::ThreadRng;

/// Pixel painting test function.
#[allow(clippy::never_loop)]
#[allow(clippy::option_expect_used)]
#[allow(clippy::single_match_else)]
#[inline]
#[must_use]
pub fn test(
    mut rng: &mut ThreadRng,
    input: &Input,
    scene: &Scene,
    mut ray: Ray,
    depth: i32,
) -> LinSrgba {
    let bump_dist = input.sett.bump_dist();
    let mut col = LinSrgba::default();
    let mut weight = 1.0_f64;
    // let mut col = palette::Srgba::new(0.0, 0.2, 1.0, 1.0).into_linear();

    if depth > input.sett.max_depth() {
        return col;
    }

    // Move rays into the grid.
    if !input.grid.boundary().contains(ray.pos()) {
        if let Some(dist) = input.grid.boundary().dist(&ray) {
            ray.travel(dist + bump_dist)
        }
    }

    if let Some((_index, voxel)) = input.grid.gen_index_voxel(ray.pos()) {
        loop {
            // Determine possible event distances.
            let voxel_dist = voxel
                .dist(&ray)
                .expect("Could not determine voxel distance.");
            let surf_hit = input.tree.observe(ray.clone(), bump_dist, voxel_dist);

            // Handle event.
            match Event::new(voxel_dist, surf_hit) {
                Event::Voxel(dist) => {
                    ray.travel(dist);
                    col += sky_col(scene, &ray, &input.cols.map()["sky"]) * weight as f32;
                    break;
                }
                Event::Surface(hit) => {
                    let group = hit.group();
                    if let Some(attr) = input.attrs.map().get(group) {
                        match attr {
                            Attributes::Mirror { abs } => {
                                ray.travel(hit.dist());
                                col += colour(&mut rng, input, scene, &ray, &hit)
                                    * *abs as f32
                                    * weight as f32;
                                weight *= 1.0 - *abs;
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
                                col += colour(&mut rng, input, scene, &ray, &hit)
                                    * *abs as f32
                                    * weight as f32;
                                weight *= 1.0 - abs;

                                let (n_curr, n_next) = if hit.side().is_inside() {
                                    (*inside, *outside)
                                } else {
                                    (*outside, *inside)
                                };
                                let crossing =
                                    Crossing::new(ray.dir(), hit.side().norm(), n_curr, n_next);
                                let trans_prob = crossing.trans_prob();
                                let trans_col = if let Some(trans_dir) = crossing.trans_dir() {
                                    let mut trans_ray = ray.clone();
                                    *trans_ray.dir_mut() = *trans_dir;
                                    trans_ray.travel(bump_dist);
                                    test(rng, input, scene, trans_ray, depth + 1)
                                } else {
                                    LinSrgba::default()
                                };

                                let ref_prob = crossing.ref_prob();
                                let mut ref_ray = ray;
                                *ref_ray.dir_mut() = *crossing.ref_dir();
                                let ref_col = test(rng, input, scene, ref_ray, depth + 1);

                                col += ((ref_col * ref_prob as f32)
                                    + (trans_col * trans_prob as f32))
                                    * weight as f32;
                                break;
                            }
                        }
                    } else {
                        ray.travel(hit.dist());
                        col += colour(&mut rng, input, scene, &ray, &hit) * weight as f32;
                        break;
                    }

                    // match hit.group() {

                    //     "glass" | "water" => {
                    //         ray.travel(hit.dist());
                    //         col += colour(&mut rng, input, scene, &ray, &hit) * 0.25;

                    //         let (n0, n1) = if hit.side().is_inside() {
                    //             (1.3, 1.0)
                    //         } else {
                    //             (1.0, 1.3)
                    //         };
                    //         let crossing = Crossing::new(ray.dir(), hit.side().norm(), n0, n1);

                    //         if let Some(trans_dir) = crossing.trans_dir() {
                    //             ray.travel(bump_dist);
                    //             *ray.dir_mut() = *trans_dir;
                    //         } else {
                    //             break;
                    //         }
                    //     }
                    //     "clouds_0" | "clouds_1" | "clouds_2" => {
                    //         ray.travel(hit.dist());
                    //         col += colour(&mut rng, input, scene, &ray, &hit) * 0.25;

                    //         let (n0, n1) = if hit.side().is_inside() {
                    //             (1.1, 1.0)
                    //         } else {
                    //             (1.0, 1.1)
                    //         };
                    //         let crossing = Crossing::new(ray.dir(), hit.side().norm(), n0, n1);

                    //         if let Some(trans_dir) = crossing.trans_dir() {
                    //             ray.travel(bump_dist);
                    //             *ray.dir_mut() = *trans_dir;
                    //         } else {
                    //             break;
                    //         }
                    //     }
                    //     "mirror" => {
                    //         ray.travel(hit.dist());
                    //         col += colour(&mut rng, input, scene, &ray, &hit) * 0.25;
                    //         *ray.dir_mut() = Crossing::init_ref_dir(
                    //             ray.dir(),
                    //             hit.side().norm(),
                    //             -ray.dir().dot(hit.side().norm()),
                    //         );
                    //         ray.travel(bump_dist);
                    //     }
                    //     _ => {
                    //         ray.travel(hit.dist());
                    //         col += colour(&mut rng, input, scene, &ray, &hit);
                    //         break;
                    //     }
                    // };
                }
            }
        }
    } else {
        col += sky_col(scene, &ray, &input.cols.map()["sky"]);
    }

    col
}

/// Perform a colouring.
#[inline]
fn colour(rng: &mut ThreadRng, input: &Input, scene: &Scene, ray: &Ray, hit: &Hit) -> LinSrgba {
    let light = (illumination::light(
        scene.light().sun_pos(),
        scene.cam().focus().orient().pos(),
        ray,
        hit,
    ) + 0.5)
        .min(1.0);
    let shadow = illumination::shadow(input, scene, ray, hit, input.sett.bump_dist(), rng);

    let sun_dir = Dir3::new_normalize(ray.pos() - scene.light().sun_pos());

    let base_col = input.cols.map()[hit.group()].get(hit.side().norm().dot(&sun_dir).abs() as f32);
    let grad = palette::Gradient::new(vec![palette::LinSrgba::default(), base_col]);

    grad.get((light * shadow) as f32)
}

/// Determine the sky colour.
#[inline]
#[must_use]
fn sky_col(
    scene: &Scene,
    ray: &Ray,
    grad: &palette::Gradient<palette::LinSrgba>,
) -> palette::LinSrgba {
    let u = (ray.dir().dot(scene.cam().focus().orient().up()) + 1.0) * 0.5;
    let v = (ray.dir().dot(scene.cam().focus().orient().right()) + 1.0) * 0.5;

    let x = (scene.perlin().sample(u, v) + 1.0) * 0.5;

    let col = grad.get(x as f32);

    // palette::Gradient::new(vec![palette::LinSrgba::default(), col]).get(1.0 - x.powi(2) as f32)
    palette::Gradient::new(vec![palette::LinSrgba::default(), col]).get(1.0)
}
