//! Visibility calculation function.

use crate::{
    sim::render::{Attributes, Input},
    Crossing, Ray,
};

/// Maximum distance tested for ray visibility [m].
const MAX_VISIBILITY_DIST: f64 = 1.0e9;

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
                Attributes::Luminous { mult } => {
                    vis *= mult;
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
                }
                | Attributes::Solar {
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

                    return trans_vis + ref_vis;
                }
            }
        } else {
            return 0.0;
        }
    }

    vis
}
