//! Probability form implementation.

use crate::{Build, Error};
use attr::load;
use ndarray::Array1;
use std::path::Path;

/// Probability distribution.
#[load]
pub enum Probability {
    /// Point.
    Point(f64),
    /// Points.
    Points(Array1<f64>),
    /// Uniform range.
    Uniform(f64, f64),
    /// Gaussian distribution.
    Gaussian(f64, f64),
    /// Linear distribution.
    Linear(f64, f64),
}

impl Build for Probability {
    type Inst = crate::Probability;

    #[inline]
    fn build(self, _in_dir: &Path) -> Result<Self::Inst, Error> {
        Ok(match self {
            Self::Point(p) => Self::Inst::new_point(p),
            Self::Points(ps) => Self::Inst::new_points(ps),
            Self::Uniform(min, max) => Self::Inst::new_uniform(min, max),
            Self::Gaussian(mu, sigma) => Self::Inst::new_gaussian(mu, sigma),
            Self::Linear(m, c) => Self::Inst::new_linear(m, c),
        })
    }
}
