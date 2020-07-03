//! Naboo painter function.

use crate::{
    render::{Camera, Event, Input, Output},
    PerlinMap, Ray, Trace,
};
use rand::rngs::ThreadRng;

/// Naboo scene painter function.
#[allow(clippy::match_single_binding)]
#[allow(clippy::option_expect_used)]
#[allow(clippy::too_many_lines)]
#[inline]
pub fn naboo(
    _thread_id: usize,
    _rng: &mut ThreadRng,
    input: &Input,
    data: &mut Output,
    weight: f64,
    pixel: [usize; 2],
    mut ray: Ray,
) {
    let bump_dist = input.sett.bump_dist();

    // Move rays into the grid.
    if !input.grid.boundary().contains(ray.pos()) {
        if let Some(dist) = input.grid.boundary().dist(&ray) {
            ray.travel(dist + bump_dist)
        }
    }

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
                    match hit.group() {
                        _ => {
                            ray.travel(hit.dist() + bump_dist);
                        }
                    };
                }
            }
        } else {
            data.image[pixel] +=
                sky_col(input.cam, input.perl, &input.cols.map()["sky"], &ray) * weight as f32;
            break;
        }
    }
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
    let u = (ray.dir().dot(cam.focus().orient().up()) + 1.0) * 0.4999;
    let v = (ray.dir().dot(cam.focus().orient().right()) + 1.0) * 0.4999;

    let x = map.sample(u, v);

    let col = grad.get(x as f32);

    // palette::Gradient::new(vec![palette::LinSrgba::default(), col]).get(1.0 - x.powi(2) as f32)
    palette::Gradient::new(vec![palette::LinSrgba::default(), col]).get(1.0)
}
