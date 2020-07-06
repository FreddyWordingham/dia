//! Pixel painter functions.

use crate::{
    render::{Event, Input, Scene},
    Ray, Trace,
};
use palette::LinSrgba;
use rand::rngs::ThreadRng;

/// Pixel painting test function.
#[allow(clippy::never_loop)]
#[allow(clippy::option_expect_used)]
#[inline]
#[must_use]
pub fn test(
    _rng: &mut ThreadRng,
    input: &Input,
    _scene: &Scene,
    mut ray: Ray,
    depth: i32,
) -> LinSrgba {
    let bump_dist = input.sett.bump_dist();

    // let mut col = LinSrgba::default();
    let mut col = palette::Srgba::new(0.0, 0.2, 1.0, 1.0).into_linear();

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
                    ray.travel(hit.dist() + bump_dist);
                    col += palette::Srgba::new(0.0, 0.2, 1.0, 1.0).into_linear();
                    break;
                }
            }
        }
    } else {
        // col += sky_col(input.cam, input.perl, &input.cols.map()["sky"], &ray) * weight as f32;
    }

    col
}
