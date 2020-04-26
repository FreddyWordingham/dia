//! Scan methods.

use crate::{Hit, Ray, Scan, Trace};

impl<'a> super::Adaptive<'a> {
    /// Scan for hits within the cell.
    #[inline]
    #[must_use]
    pub fn hit_scan(&self, ray: &Ray, bump_dist: f64) -> Scan {
        debug_assert!(self.boundary().contains(ray.pos()));
        debug_assert!(bump_dist > 0.0);

        match self {
            Self::Leaf { boundary, tris } => {
                let mut nearest: Option<Hit> = None;
                for (group, tri) in tris {
                    if let Some((dist, side)) = tri.dist_side(ray) {
                        if nearest.is_none() || (dist < nearest.as_ref().unwrap().dist()) {
                            nearest = Some(Hit::new(*group, dist, side));
                        }
                    }
                }

                let boundary_dist = boundary.dist(ray).unwrap();
                if let Some(hit) = nearest {
                    if hit.dist() < (boundary_dist - bump_dist) {
                        return Scan::new_surface(hit);
                    } else if hit.dist() < (boundary_dist + bump_dist) {
                        return Scan::new_both(hit, boundary_dist);
                    }
                }

                Scan::Boundary {
                    dist: boundary_dist,
                }
            }
            Self::Empty { boundary } => Scan::Boundary {
                dist: boundary.dist(ray).unwrap(),
            },
            Self::Root { .. } | Self::Branch { .. } => {
                panic!("Should not be performing hit scans on branching cells!");
            }
        }
    }
}
