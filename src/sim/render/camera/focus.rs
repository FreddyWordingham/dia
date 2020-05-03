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

        let tar_dist = nalgebra::distance(&pos, &tar);
        let dof = if let Some((samples, angle)) = dof {
            Some((samples, tar_dist * angle.tan()))
        } else {
            None
        };

        Self {
            orient: Orient::new(Ray::new(pos, Dir3::new_normalize(tar - pos))),
            tar,
            dof,
        }
    }

    /// Calculate the nth depth-of-field observation position.
    #[inline]
    #[must_use]
    pub fn observation_pos(&self, offset: f64, n: i32) -> Pos3 {
        let mut pos = self.orient.pos().clone();

        if let Some((dof_samples, max_rad)) = self.dof {
            let (rho, mut theta) = golden::circle(n, dof_samples);
            theta += offset;

            pos += self.orient.forward().as_ref()
                * nalgebra::distance(self.orient.pos(), &self.tar)
                * (1.0 - theta.cos()); // TODO: See what happens when you don't bum forward.
            pos += self.orient.right().as_ref() * theta.sin() * max_rad * rho;
            pos += self.orient.up().as_ref() * theta.cos() * max_rad * rho;
        }

        pos
    }

    /// Calculate the nth depth-of-field observation ray.
    #[inline]
    #[must_use]
    pub fn observation_ray(&self, offset: f64, n: i32) -> Ray {
        debug_assert!(n >= 0);

        let pos = self.observation_pos(offset, n);
        Ray::new(pos, Dir3::new_normalize(self.tar - self.orient.pos()))
    }
}
