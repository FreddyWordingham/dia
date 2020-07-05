//! Kessler painter function.

use crate::{
    render::{illumination, Event, Input, Output},
    Crossing, Dir3, PerlinMap, Ray, Trace, Vec3,
};
use rand::rngs::ThreadRng;

/// Kessler scene painter function.
#[allow(clippy::match_single_binding)]
#[allow(clippy::option_expect_used)]
#[allow(clippy::too_many_lines)]
#[inline]
pub fn kessler(
    _thread_id: usize,
    mut rng: &mut ThreadRng,
    input: &Input,
    data: &mut Output,
    mut weight: f64,
    pixel: [usize; 2],
    mut ray: Ray,
) {
    let sun_dir = Dir3::new_normalize(-Vec3::new(
        input.sett.sun_pos().x,
        input.sett.sun_pos().y,
        input.sett.sun_pos().z,
    ));
    let bump_dist = input.sett.bump_dist();

    // Move rays into the grid.
    if !input.grid.boundary().contains(ray.pos()) {
        if let Some(dist) = input.grid.boundary().dist(&ray) {
            ray.travel(dist + bump_dist)
        }
    }

    loop {
        // Check if inside the grid.
        if let Some((index, voxel)) = input.grid.gen_index_voxel(ray.pos()) {
            // Determine possible event distances.
            let voxel_dist = voxel
                .dist(&ray)
                .expect("Could not determine voxel distance.");
            let surf_hit = input.tree.observe(ray.clone(), bump_dist, voxel_dist);

            // Handle event.
            match Event::new(voxel_dist, surf_hit) {
                Event::Voxel(dist) => ray.travel(dist + bump_dist),
                Event::Surface(hit) => {
                    match hit.group() {
                        "blaze" => {
                            ray.travel(hit.dist());

                            let w = (ray.pos().x.powi(2) + (ray.pos().y - 0.9).powi(2)).sqrt()
                                / ((ray.pos().z - 3.0).sqrt() / 0.75);

                            if w <= 1.0 {
                                let light = illumination::light(
                                    input.sett.sun_pos(),
                                    input.cam.focus().orient().pos(),
                                    &ray,
                                    &hit,
                                );
                                let shadow =
                                    illumination::shadow(input, &ray, &hit, bump_dist, &mut rng);

                                let base_col = input.cols.map()[hit.group()]
                                    .get(hit.side().norm().dot(&sun_dir).abs() as f32);
                                let grad = palette::Gradient::new(vec![
                                    palette::LinSrgba::default(),
                                    base_col,
                                ]);

                                data.image[pixel] += grad.get((light * shadow) as f32)
                                    * weight as f32
                                    * (1.0 - w) as f32;

                                data.hits[index] += weight;

                                weight *= 0.9;
                            }

                            ray.travel(bump_dist);
                        }
                        "planet_clouds" => {
                            ray.travel(hit.dist());
                            let light = illumination::light(
                                input.sett.sun_pos(),
                                input.cam.focus().orient().pos(),
                                &ray,
                                &hit,
                            );
                            let shadow =
                                illumination::shadow(input, &ray, &hit, bump_dist, &mut rng);

                            let base_col = input.cols.map()[hit.group()]
                                .get(hit.side().norm().dot(&sun_dir).abs() as f32);
                            let grad = palette::Gradient::new(vec![
                                palette::LinSrgba::default(),
                                base_col,
                            ]);

                            data.image[pixel] += grad.get((light * shadow) as f32) * weight as f32;

                            data.hits[index] += weight;

                            weight *= 0.75;
                            ray.travel(bump_dist);
                        }
                        "starfield" | "solar" => {
                            ray.travel(hit.dist());
                            let light = illumination::light(
                                input.sett.sun_pos(),
                                input.cam.focus().orient().pos(),
                                &ray,
                                &hit,
                            );
                            let shadow =
                                illumination::shadow(input, &ray, &hit, bump_dist, &mut rng).sqrt();

                            let base_col = input.cols.map()[hit.group()]
                                .get(hit.side().norm().dot(&sun_dir).abs() as f32);
                            let grad = palette::Gradient::new(vec![
                                palette::LinSrgba::default(),
                                base_col,
                            ]);

                            data.image[pixel] += grad.get((light * shadow) as f32) * weight as f32;

                            data.hits[index] += weight;
                            break;
                        }
                        "visor" => {
                            ray.travel(hit.dist());
                            data.image[pixel] += palette::LinSrgba::default();
                            *ray.dir_mut() = Crossing::init_ref_dir(
                                ray.dir(),
                                hit.side().norm(),
                                -ray.dir().dot(hit.side().norm()),
                            );
                            ray.travel(bump_dist);
                        }
                        _ => {
                            ray.travel(hit.dist());
                            let light = (illumination::light(
                                input.sett.sun_pos(),
                                input.cam.focus().orient().pos(),
                                &ray,
                                &hit,
                            ) + 0.5)
                                .min(1.0);
                            let shadow =
                                illumination::shadow(input, &ray, &hit, bump_dist, &mut rng);

                            let base_col = input.cols.map()[hit.group()]
                                .get(hit.side().norm().dot(&sun_dir).abs() as f32);
                            let grad = palette::Gradient::new(vec![
                                palette::LinSrgba::default(),
                                base_col,
                            ]);

                            data.image[pixel] += grad.get((light * shadow) as f32) * weight as f32;

                            data.hits[index] += weight;
                            break;
                        }
                    };
                }
            }
        } else {
            data.image[pixel] +=
                sky_col(input.perl, &input.cols.map()["sky"], &ray) * weight as f32;
            break;
        }
    }
}

/// Determine the sky colour.
#[inline]
#[must_use]
fn sky_col(
    map: &PerlinMap,
    grad: &palette::Gradient<palette::LinSrgba>,
    ray: &Ray,
) -> palette::LinSrgba {
    let u = ray.dir().x.abs();
    let v = ray.dir().y.abs();

    let col = grad.get(map.sample(u, v) as f32);

    palette::Gradient::new(vec![palette::LinSrgba::default(), col]).get(0.25)
}
