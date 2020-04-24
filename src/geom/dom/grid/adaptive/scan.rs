//! Scan methods.

impl<'a> super::Adaptive<'a> {
    // /// Scan for hits within the cell.
    // #[inline]
    // #[must_use]
    // pub fn hit_scan(&self, ray: &Ray, bump_dist: f64) -> Scan {
    //     debug_assert!(self.boundary().contains(ray.pos()));
    //     debug_assert!(bump_dist > 0.0);

    //     match self {
    //         Self::Leaf { boundary, tris } => {
    //             let mut nearest: Option<Hit> = None;
    //             for (group, tri) in tris {
    //                 if let Some((dist, side)) = tri.dist_side(ray) {
    //                     if nearest.is_none() || (nearest.as_ref().unwrap().dist() > dist) {
    //                         nearest = Some(Hit::new(*group, dist, side));
    //                     }
    //                 }
    //             }

    //             let boundary_dist = boundary.dist(ray).unwrap();
    //             if let Some(hit) = nearest {
    //                 if hit.dist() < (boundary_dist - bump_dist) {
    //                     return Scan::Surface { hit };
    //                 } else if hit.dist() < (boundary_dist + bump_dist) {
    //                     return Scan::Both {
    //                         hit,
    //                         dist: boundary_dist,
    //                     };
    //                 }
    //             }

    //             Scan::Boundary {
    //                 dist: boundary_dist,
    //             }
    //         }
    //         Self::Empty { boundary } => Scan::Boundary {
    //             dist: boundary.dist(ray).unwrap(),
    //         },
    //         Self::Root { .. } | Self::Branch { .. } => {
    //             panic!("Should not be performing hit scans on branching cells!");
    //         }
    //     }
    // }

    // /// Determine what a ray would observe within the cell.
    // #[inline]
    // #[must_use]
    // pub fn observe(&self, mut ray: Ray, bump_dist: f64) -> Option<Hit> {
    //     debug_assert!(bump_dist > 0.0);

    //     let mut dist_travelled = 0.0;

    //     // Move the ray to within the domain of the grid if it isn't already within it.
    //     if !self.boundary().contains(ray.pos()) {
    //         if let Some(dist) = self.boundary().dist(&ray) {
    //             let d = dist + bump_dist;
    //             ray.travel(d);
    //             dist_travelled += d;
    //         } else {
    //             return None;
    //         }
    //     }

    //     // Trace forward until leaving the grid or observing something.
    //     if !self.boundary().contains(ray.pos()) {
    //         return None;
    //     }
    //     while let Some(cell) = self.find_terminal_cell(ray.pos()) {
    //         match cell.hit_scan(&ray, bump_dist) {
    //             Scan::Surface { mut hit } | Scan::Both { mut hit, .. } => {
    //                 *hit.dist_mut() += dist_travelled;
    //                 return Some(hit);
    //             }
    //             Scan::Boundary { dist } => {
    //                 let d = dist + bump_dist;
    //                 ray.travel(d);
    //                 dist_travelled += d;

    //                 if !self.boundary().contains(ray.pos()) {
    //                     return None;
    //                 }
    //             }
    //         }
    //     }

    //     None
    // }
}
