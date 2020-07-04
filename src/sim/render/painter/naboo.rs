//! Naboo painter function.

use crate::{
    render::{illumination, Camera, Event, Input, Output},
    Crossing, Dir3, Hit, PerlinMap, Ray, Trace,
};
use rand::rngs::ThreadRng;
use std::sync::{Arc, Mutex};

/// Naboo scene painter function.
#[allow(clippy::match_single_binding)]
#[allow(clippy::option_expect_used)]
#[allow(clippy::too_many_lines)]
#[inline]
pub fn naboo(
    thread_id: usize,
    mut rng: &mut ThreadRng,
    input: &Input,
    mut data: &Arc<Mutex<Output>>,
    mut weight: f64,
    pixel: [usize; 2],
    mut ray: Ray,
) {
    if weight < 0.01 {
        return;
    }

    let bump_dist = input.sett.bump_dist();

    // Move rays into the grid.
    if !input.grid.boundary().contains(ray.pos()) {
        if let Some(dist) = input.grid.boundary().dist(&ray) {
            ray.travel(dist + bump_dist)
        }
    }

    let mut fog = 0.0;
    loop {
        // Check if inside the grid.
        if let Some((_index, voxel)) = input.grid.gen_index_voxel(ray.pos()) {
            // Determine possible event distances.
            let voxel_dist = voxel
                .dist(&ray)
                .expect("Could not determine voxel distance.");
            let surf_hit = input.tree.observe(ray.clone(), bump_dist, voxel_dist);

            // Handle event.
            match Event::new(voxel_dist, surf_hit) {
                Event::Voxel(dist) => ray.travel(dist + bump_dist),
                Event::Surface(hit) => {
                    // if hit.dist() > 0.1 {
                    //     fog += 0.1
                    //         * illumination::visibility(
                    //             0,
                    //             input,
                    //             Ray::new(
                    //                 *ray.pos(),
                    //                 Dir3::new_normalize(ray.pos() - input.sett.sun_pos()),
                    //             ),
                    //             bump_dist,
                    //         );
                    //     ray.travel(0.1);
                    // } else {
                    match hit.group() {
                        "water" => {
                            ray.travel(hit.dist());

                            colour(input, &ray, &hit, data, weight * 0.1, pixel, &mut rng);
                            weight *= 0.9;

                            fog = 0.0;

                            let crossing = Crossing::new(ray.dir(), hit.side().norm(), 1.0, 1.75);

                            let trans_prob = crossing.trans_prob();
                            if let Some(trans_dir) = crossing.trans_dir() {
                                let mut trans_ray = ray.clone();
                                *trans_ray.dir_mut() = *trans_dir;
                                trans_ray.travel(bump_dist);
                                naboo(
                                    thread_id,
                                    rng,
                                    input,
                                    data,
                                    weight * trans_prob,
                                    pixel,
                                    trans_ray,
                                );
                            }

                            let ref_prob = crossing.ref_prob();
                            weight *= ref_prob;
                            *ray.dir_mut() = *crossing.ref_dir();
                            ray.travel(bump_dist);
                        }
                        "clouds_0" | "clouds_1" | "clouds_2" => {
                            ray.travel(hit.dist());

                            colour(input, &ray, &hit, data, weight * 0.2, pixel, &mut rng);
                            weight *= 0.8;

                            let crossing = Crossing::new(ray.dir(), hit.side().norm(), 1.0, 1.1);

                            if let Some(trans_dir) = crossing.trans_dir() {
                                ray.travel(bump_dist);
                                *ray.dir_mut() = *trans_dir;
                            } else {
                                break;
                            }
                        }
                        "mirror" => {
                            ray.travel(hit.dist());
                            data.lock().unwrap().image[pixel] += palette::LinSrgba::default();
                            *ray.dir_mut() = Crossing::init_ref_dir(
                                ray.dir(),
                                hit.side().norm(),
                                -ray.dir().dot(hit.side().norm()),
                            );
                            ray.travel(bump_dist);
                        }
                        _ => {
                            ray.travel(hit.dist());
                            colour(input, &ray, &hit, data, weight, pixel, &mut rng);
                            break;
                        }
                    };
                    // }
                }
            }
        } else {
            data.lock().unwrap().image[pixel] +=
                sky_col(input.cam, input.perl, &input.cols.map()["sky"], &ray) * weight as f32;
            break;
        }

        // println!("fog: {}", fog);
        data.lock().unwrap().image[pixel] +=
            input.cols.map()["sky"].get(0.0) * (weight * fog * 0.00_000_5) as f32;
    }
}

/// Perform a colouring.
#[inline]
fn colour(
    input: &Input,
    ray: &Ray,
    hit: &Hit,
    mut data: &Arc<Mutex<Output>>,
    weight: f64,
    pixel: [usize; 2],
    rng: &mut ThreadRng,
) {
    let light = (illumination::light(
        input.sett.sun_pos(),
        input.cam.focus().orient().pos(),
        ray,
        hit,
    ) + 0.5)
        .min(1.0);
    let shadow = illumination::shadow(input, ray, hit, input.sett.bump_dist(), rng);

    let sun_dir = Dir3::new_normalize(ray.pos() - input.sett.sun_pos());

    let base_col = input.cols.map()[hit.group()].get(hit.side().norm().dot(&sun_dir).abs() as f32);
    let grad = palette::Gradient::new(vec![palette::LinSrgba::default(), base_col]);

    data.lock().unwrap().image[pixel] += grad.get((light * shadow) as f32) * weight as f32;
}

/// Determine the sky colour.
#[inline]
#[must_use]
fn sky_col(
    cam: &Camera,
    map: &PerlinMap,
    grad: &palette::Gradient<palette::LinSrgba>,
    ray: &Ray,
) -> palette::LinSrgba {
    let u = (ray.dir().dot(cam.focus().orient().up()) + 1.0) * 0.5;
    let v = (ray.dir().dot(cam.focus().orient().right()) + 1.0) * 0.5;

    let x = (map.sample(u, v) + 1.0) * 0.5;

    let col = grad.get(x as f32);

    // palette::Gradient::new(vec![palette::LinSrgba::default(), col]).get(1.0 - x.powi(2) as f32)
    palette::Gradient::new(vec![palette::LinSrgba::default(), col]).get(1.0)
}
