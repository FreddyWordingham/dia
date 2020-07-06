//! Pixel painter functions.

use crate::{
    render::{Event, Input, Scene},
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
                    ray.travel(dist + bump_dist);
                    break;
                }
                Event::Surface(hit) => {
                    match hit.group() {
                        "glass" => {
                            ray.travel(hit.dist());
                            col += colour(&mut rng, input, scene, &ray, &hit) * 0.25;

                            let (n0, n1) = if hit.side().is_inside() {
                                (1.1, 1.0)
                            } else {
                                (1.0, 1.1)
                            };
                            let crossing = Crossing::new(ray.dir(), hit.side().norm(), n0, n1);

                            if let Some(trans_dir) = crossing.trans_dir() {
                                ray.travel(bump_dist);
                                *ray.dir_mut() = *trans_dir;
                            } else {
                                break;
                            }
                        }
                        "mirror" => {
                            ray.travel(hit.dist());
                            *ray.dir_mut() = Crossing::init_ref_dir(
                                ray.dir(),
                                hit.side().norm(),
                                -ray.dir().dot(hit.side().norm()),
                            );
                            ray.travel(bump_dist);
                        }
                        _ => {
                            ray.travel(hit.dist());
                            col += colour(&mut rng, input, scene, &ray, &hit);
                            break;
                        }
                    };
                }
            }
        }
    } else {
        // col += sky_col(input.cam, input.perl, &input.cols.map()["sky"], &ray) * weight as f32;
    }

    col
}

/// Perform a colouring.
#[inline]
fn colour(_rng: &mut ThreadRng, input: &Input, scene: &Scene, ray: &Ray, hit: &Hit) -> LinSrgba {
    let light = 1.0;
    let shadow = 1.0;

    let sun_dir = Dir3::new_normalize(ray.pos() - scene.light().sun_pos());

    let base_col = input.cols.map()[hit.group()].get(hit.side().norm().dot(&sun_dir).abs() as f32);
    let grad = palette::Gradient::new(vec![palette::LinSrgba::default(), base_col]);

    grad.get((light * shadow) as f32)
}
