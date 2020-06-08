//! Field painter function.

use crate::{
    render::{Event, Input, Output},
    Ray, Trace, Vec3,
};
use rand::rngs::ThreadRng;

/// Test painter function.
#[allow(clippy::option_expect_used)]
#[inline]
pub fn field(
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
                    ray.travel(hit.dist() + bump_dist);
                    let base_col = input.cols.map()["greyscale"]
                        .get(hit.side().norm().dot(&Vec3::z_axis()) as f32);
                    let grad = palette::Gradient::new(vec![palette::LinSrgba::default(), base_col]);

                    data.image[pixel] += grad.get(1.0) * weight as f32;

                    data.hits[index] += weight;
                    break;
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
