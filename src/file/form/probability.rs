//! Probability form implementation.

use crate::{display_field, report, Build, Error};
use attr::load;
use ndarray::Array1;
use std::{
    fmt::{Display, Formatter},
    path::Path,
};

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
    /// Constant spline.
    ConstantSpline(Vec<f64>, Vec<f64>),
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
            Self::ConstantSpline(xs, ps) => {
                Self::Inst::new_constant_spline(Array1::from(xs), &Array1::from(ps))
            }
        })
    }
}

impl Display for Probability {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        let kind = match self {
            Self::Point { .. } => "Constant",
            Self::Points { .. } => "Line",
            Self::Uniform { .. } => "Bifurcation",
            Self::Gaussian { .. } => "Gaussian",
            Self::ConstantSpline { .. } => "Constant Spline",
        };
        display_field!(fmt, "[kind]", kind)
    }
}
