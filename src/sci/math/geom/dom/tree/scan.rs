//! Hit-scan methods.

use crate::{tree::Cell, Hit, Ray, Trace};

impl<'a> Cell<'a> {
    /// Determine what a ray would observe within the cell.
    #[inline]
    #[must_use]
    pub fn observe(&self, mut ray: Ray, bump_dist: f64, max_dist: f64) -> Option<Hit> {
        debug_assert!(bump_dist > 0.0);
        debug_assert!(max_dist > 0.0);

        let mut dist_travelled = 0.0;

        // Move the ray to within the domain of the tree if it isn't already within it.
        if !self.boundary().contains(ray.pos()) {
            if let Some(dist) = self.boundary().dist(&ray) {
                let d = dist + bump_dist;
                ray.travel(d);
                dist_travelled += d;
            } else {
                return None;
            }
        }

        // Illumination at limbs of the tree can lead to piercing in and out again.
        // if !self.boundary().contains(ray.pos()) {
        //     return None;
        // }

        // while let Some(cell) = self.find_terminal_cell(ray.pos()) {
        //     match cell.hit_scan(&ray, bump_dist) {
        //         Scan::Surface { mut hit } | Scan::Both { mut hit, .. } => {
        //             *hit.dist_mut() += dist_travelled;
        //             return Some(hit);
        //         }
        //         Scan::Boundary { dist } => {
        //             let d = dist + bump_dist;
        //             ray.travel(d);
        //             dist_travelled += d;

        //             if !self.boundary().contains(ray.pos()) {
        //                 return None;
        //             }
        //         }
        //     }
        // }

        None
    }
}
