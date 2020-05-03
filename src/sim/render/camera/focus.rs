//! Focus implementation.

use crate::{access, clone, golden, Dir3, Orient, Pos3, Ray};

/// Focus structure.
pub struct Focus {
    /// Orientation.
    orient: Orient,
    /// Target point.
    tar: Pos3,
    /// Optional depth-of-field samples and maximum sampling radius.
    dof: Option<(i32, f64)>,
}

impl Focus {
    access!(orient, Orient);
    access!(tar, Pos3);
    clone!(dof, Option<(i32, f64)>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(pos: Pos3, tar: Pos3, dof: Option<(i32, f64)>) -> Self {
        debug_assert!(dof.is_none() || dof.unwrap().0 > 0);
        debug_assert!(dof.is_none() || dof.unwrap().1 > 0.0);

        let tar_dist = nalgebra::distance(pos, tar);
        let dof = if let Some((samples, angle)) = dof {
            Some((samples, tar_dist * angle.tan()))
        } else {
            None
        };

        Self {
            orient: Orient::new(Ray::new(pos, Dir3::new_normalize(tar - pos))),
            tar,
        }
    }

    /// Calculate the nth depth-of-field observation position.
    #[inline]
    #[must_use]
    pub fn observation_pos(&self, n: usize) -> Pos3 {
        let pos = self.orient.pos();

        if let Some((dof_samples, max_rad)) = self.dof {
            let (rho, theta) = golden::circle(n, dof_samples);
        } else {
        }
    }

    /// Calculate the nth depth-of-field observation ray.
    #[inline]
    #[must_use]
    pub fn observation_ray(&self, n: usize) -> Ray {
        let pos = self.observation_pos(n);
        Ray::new(pos, Dir3::new_normalize(self.tar - self.orient.pos()));
    }
}
