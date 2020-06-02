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
    /// Constant spline.
    ConstantSpline {
        /// Cumulative distribution function.
        cdf: Formula,
    },
    /// Linear spline.
    LinearSpline {
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

    /// Construct a new constant spline instance.
    #[inline]
    #[must_use]
    pub fn new_constant_spline(xs: Array1<f64>, ps: &Array1<f64>) -> Self {
        debug_assert!(xs.len() > 1);
        debug_assert!(xs.len() == (ps.len() + 1));
        debug_assert!(ps.iter().all(|p| *p >= 0.0));

        let mut cdf = Vec::with_capacity(ps.len());
        let mut total = 0.0;
        cdf.push(total);
        for ((x_curr, x_next), prob) in xs.iter().zip(xs.iter().skip(1)).zip(ps.iter()) {
            let area = (x_next - x_curr) * prob;
            total += area;
            cdf.push(total);
        }
        let mut cdf = Array1::from(cdf);
        cdf /= total;

        Self::ConstantSpline {
            cdf: Formula::new_linear_spline_auto(cdf, xs),
        }
    }

    /// Construct a new linear spline instance.
    /// Determine the cumulative distribution function for a given
    #[inline]
    #[must_use]
    pub fn new_linear_spline(xs: Array1<f64>, ps: &Array1<f64>) -> Self {
        debug_assert!(xs.len() > 1);
        debug_assert!(xs.len() == ps.len());
        debug_assert!(ps.iter().all(|x| *x >= 0.0));

        let mut grads = Vec::with_capacity(xs.len());
        let mut quads = Vec::with_capacity(xs.len());

        let mut cdf = Vec::with_capacity(ps.len());
        let mut total = 0.0;
        cdf.push(total);
        for ((x_curr, x_next), (prob_curr, prob_next)) in xs
            .iter()
            .zip(xs.iter().skip(1))
            .zip(ps.iter().zip(ps.iter().skip(1)))
        {
            let area = (x_next - x_curr) * (prob_next + prob_curr) * 0.5;
            total += area;
            cdf.push(total);

            grads.push(*x_curr);
            quads.push((prob_next - prob_curr) / (x_next - x_curr));
        }
        let mut cdf = Array1::from(cdf);
        cdf /= total;

        // let mut grads = Vec::with_capacity(xs.len());
        // let mut quads = Vec::with_capacity(xs.len());
        // for ((x_curr, x_next), (cdf_curr, cdf_next)) in xs
        //     .iter()
        //     .zip(xs.iter().skip(1))
        //     .zip(cdf.iter().zip(cdf.iter().skip(1)))
        // {
        //     grads.push(3.0);
        //     quads.push(1.0);
        // }

        Self::LinearSpline {
            cdf: Formula::new_quadratic_spline(cdf, xs, Array1::from(grads), Array1::from(quads)),
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
            Self::ConstantSpline { cdf } => cdf.y(rng.gen()),
            Self::LinearSpline { cdf } => cdf.y(rng.gen()),
        }
    }
}
