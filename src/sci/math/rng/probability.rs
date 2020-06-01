//! Probability distribution implementation.

use crate::{distribution, Formula};
use ndarray::Array1;
use rand::{rngs::ThreadRng, Rng};

/// Probability distribution formulae.
pub enum Probability {
    /// Point.
    Point {
        /// Constant value.
        c: f64,
    },
    /// Points.
    Points {
        /// Possible values.
        cs: Array1<f64>,
    },
    /// Uniform range.
    Uniform {
        /// Minimum value.
        min: f64,
        /// Maximum value.
        max: f64,
    },
    /// Gaussian distribution.
    Gaussian {
        /// Average value.
        mu: f64,
        /// Variance.
        sigma: f64,
    },
    /// Constant interpolation.
    ConstantInterpolation {
        /// Cumulative distribution function.
        cdf: Formula,
    },
}

impl Probability {
    /// Construct a new point instance.
    #[inline]
    #[must_use]
    pub const fn new_point(c: f64) -> Self {
        Self::Point { c }
    }

    /// Construct a new points instance.
    #[inline]
    #[must_use]
    pub fn new_points(cs: Array1<f64>) -> Self {
        debug_assert!(cs.len() > 1);
        Self::Points { cs }
    }

    /// Construct a new uniform instance.
    #[inline]
    #[must_use]
    pub fn new_uniform(min: f64, max: f64) -> Self {
        debug_assert!(min < max);
        Self::Uniform { min, max }
    }

    /// Construct a new gaussian instance.
    #[inline]
    #[must_use]
    pub fn new_gaussian(mu: f64, sigma: f64) -> Self {
        debug_assert!(sigma > 0.0);
        Self::Gaussian { mu, sigma }
    }

    /// Construct a new constant interpolation instance.
    #[inline]
    #[must_use]
    pub fn new_constant_interpolation(xs: Array1<f64>, ps: &Array1<f64>) -> Self {
        debug_assert!(xs.len() > 1);
        debug_assert!(xs.len() == (ps.len() + 1));
        debug_assert!(ps.iter().all(|x| *x >= 0.0));

        let mut cdf = Vec::with_capacity(ps.len());
        let mut total = 0.0;
        cdf.push(total);
        for ((x_curr, x_next), prob) in xs.iter().zip(xs.iter().skip(1)).zip(ps.iter()) {
            let area = (x_next - x_curr) * prob;
            total += area;
            cdf.push(total);
        }
        let mut cdf = Array1::from(cdf);
        cdf /= cdf[cdf.len() - 1];

        Self::ConstantInterpolation {
            cdf: Formula::new_linear_spline_auto(cdf, xs),
        }
    }

    /// Generate a random number from the described distribution.
    #[inline]
    #[must_use]
    pub fn gen(&self, rng: &mut ThreadRng) -> f64 {
        match self {
            Self::Point { c } => *c,
            Self::Points { cs } => cs[rng.gen_range(0, cs.len())],
            Self::Uniform { min, max } => rng.gen_range(*min, *max),
            Self::Gaussian { mu, sigma } => distribution::gaussian(rng, *mu, *sigma),
            Self::ConstantInterpolation { cdf } => cdf.y(rng.gen()),
        }
    }
}
