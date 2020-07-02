//! Probability distribution implementation.

use crate::{display_field, tools::lerp, Dir2, Vec2, X, Y};
use ndarray::{Array2, Axis};
use rand::{rngs::ThreadRng, Rng};
use std::{
    f64::consts::PI,
    fmt::{Display, Formatter, Result},
};

/// Perlin noise map structure.
pub struct PerlinMap {
    /// Gradient vectors.
    grads: Array2<Dir2>,
}

impl PerlinMap {
    /// Construct a new instance.
    #[allow(clippy::result_expect_used)]
    #[inline]
    #[must_use]
    pub fn new(res: [usize; 2], rng: &mut ThreadRng) -> Self {
        debug_assert!(res[X] > 0);
        debug_assert!(res[Y] > 0);

        let mut grads = Vec::with_capacity(res[X] * res[Y]);
        for _ in 0..res[X] {
            for _ in 0..res[Y] {
                let theta = rng.gen_range(0.0, 2.0 * PI);
                let x = theta.cos();
                let y = theta.sin();
                grads.push(Dir2::new_normalize(Vec2::new(x, y)));
            }
        }

        Self {
            grads: Array2::from_shape_vec(res, grads).expect("Could not create gradient array."),
        }
    }

    /// Sample a point in the map.
    #[inline]
    #[must_use]
    pub fn sample(&self, x: f64, y: f64) -> f64 {
        debug_assert!(x >= 0.0);
        debug_assert!(y >= 0.0);
        debug_assert!(x <= 1.0);
        debug_assert!(y <= 1.0);

        let nx = self.grads.len_of(Axis(X));
        let ny = self.grads.len_of(Axis(Y));

        let ix = (x * (nx - 1) as f64).floor();
        let iy = (y * (ny - 1) as f64).floor();

        let u = (x * nx as f64) - ix;
        let v = (y * ny as f64) - iy;

        let dist_a = Vec2::new(u, v - 1.0);
        let dist_b = Vec2::new(u - 1.0, v - 1.0);
        let dist_c = Vec2::new(u - 1.0, v);
        let dist_d = Vec2::new(u, v);

        let a = dist_a.dot(&self.grads[[nx, ny + 1]]);
        let b = dist_b.dot(&self.grads[[nx + 1, ny + 1]]);
        let c = dist_c.dot(&self.grads[[nx + 1, ny]]);
        let d = dist_d.dot(&self.grads[[nx, ny]]);

        let x0 = lerp(a, b, u);
        let x1 = lerp(d, c, u);

        lerp(x0, x1, v)
    }
}

impl Display for PerlinMap {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        display_field!(
            fmt,
            "resolution",
            format!("{} x {}", self.grads.shape()[X], self.grads.shape()[Y])
        )
    }
}
