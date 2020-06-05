//! Test painter function.

use crate::{
    render::{Event, Input, Output},
    Ray, Trace,
};
use rand::rngs::ThreadRng;

/// Test painter function.
#[allow(clippy::option_expect_used)]
#[inline]
pub fn test(
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

    // Loop while inside the grid.
    while let Some((index, voxel)) = input.grid.gen_index_voxel(ray.pos()) {
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
                let grad = match hit.group() {
                    "ground" => "greens",
                    "tree" => "reds",
                    "leaves" => "blues",
                    _ => {
                        panic!("Unknown hit group {}", hit.group());
                    }
                };
                data.image[pixel] += input.cols.map()[grad].get(0.0) * weight as f32;

                data.hits[index] += weight;
                break;
            }
        }
    }
}
