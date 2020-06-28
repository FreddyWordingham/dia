//! Probability distribution implementation.

use crate::{display_field, Dir2, Vec2, X, Y};
use ndarray::Array2;
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
