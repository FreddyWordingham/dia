//! Field painter function.

use crate::{
    render::{illumination, Event, Input, Output},
    Crossing, Dir3, Ray, Trace, Vec3,
};
use rand::rngs::ThreadRng;

/// Test painter function.
#[allow(clippy::option_expect_used)]
#[allow(clippy::too_many_lines)]
#[inline]
pub fn field(
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
                Event::Surface(mut hit) => {
                    match hit.group() {
                        "mirror" => {
                            ray.travel(hit.dist());
                            *ray.dir_mut() = Crossing::init_ref_dir(
                                ray.dir(),
                                hit.side().norm(),
                                -ray.dir().dot(hit.side().norm()),
                            );
                            ray.travel(bump_dist);
                            continue;
                        }
                        "leaves" => {
                            ray.travel(hit.dist());
                            let light = illumination::light(
                                input.sett.sun_pos(),
                                input.cam.focus().orient().pos(),
                                &ray,
                                &hit,
                            );
                            hit.flip_side();
                            let shadow =
                                ((illumination::shadow(input, &ray, &hit, bump_dist, &mut rng)
                                    + 0.05)
                                    * 4.0)
                                    .min(1.0);

                            let base_col = input.cols.map()[hit.group()]
                                // .get(((hit.side().norm().dot(&Vec3::x_axis()) + 1.0) * 0.5) as f32);
                                // .get(((hit.side().norm().dot(&Vec3::x_axis()) + 1.0) * 0.5) as f32);
                                .get(((hit.side().norm().dot(&sun_dir) + 1.0) * 0.5) as f32);
                            // .get(hit.side().norm().dot(&Vec3::z_axis()).abs() as f32);
                            let grad = palette::Gradient::new(vec![
                                palette::LinSrgba::default(),
                                base_col,
                            ]);

                            data.image[pixel] +=
                                grad.get((light * shadow) as f32) * (weight * 0.75) as f32;
                            data.hits[index] += weight;

                            weight *= 0.25;
                            ray.travel(bump_dist);
                        }
                        "solar" => {
                            ray.travel(hit.dist());
                            let light = illumination::light(
                                input.sett.sun_pos(),
                                input.cam.focus().orient().pos(),
                                &ray,
                                &hit,
                            );
                            let shadow = 1.0
                                - (1.0
                                    - illumination::shadow(input, &ray, &hit, bump_dist, &mut rng))
                                .powi(4);

                            let base_col = input.cols.map()[hit.group()]
                                .get(((hit.side().norm().dot(&sun_dir) + 1.0) * 0.5) as f32);
                            let grad = palette::Gradient::new(vec![
                                palette::LinSrgba::default(),
                                base_col,
                            ]);

                            data.image[pixel] += grad.get((light * shadow) as f32) * weight as f32;

                            data.hits[index] += weight;
                            break;
                        }
                        _ => {
                            ray.travel(hit.dist());
                            let light = illumination::light(
                                input.sett.sun_pos(),
                                input.cam.focus().orient().pos(),
                                &ray,
                                &hit,
                            );
                            let shadow = 1.0
                                - (1.0
                                    - illumination::shadow(input, &ray, &hit, bump_dist, &mut rng))
                                .powi(2);
                            // let shadow =
                            //     illumination::shadow(input, &ray, &hit, bump_dist, &mut rng);

                            let base_col = input.cols.map()[hit.group()]
                                // .get(hit.side().norm().dot(&sun_dir).abs() as f32);
                                .get(((hit.side().norm().dot(&sun_dir) + 1.0) * 0.5) as f32);
                            // .get(hit.side().norm().dot(&Vec3::z_axis()).abs() as f32);
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
            data.image[pixel] += sky_col(&input.cols.map()["sky"], &ray) * weight as f32;
            break;
        }
    }
}

/// Determine the sky colour.
#[inline]
#[must_use]
fn sky_col(grad: &palette::Gradient<palette::LinSrgba>, ray: &Ray) -> palette::LinSrgba {
    grad.get((ray.dir().dot(&Vec3::z_axis())).abs() as f32)
}
