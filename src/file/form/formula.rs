//! Formula form implementation.

use crate::{Build, Error};
use attr::load;
use ndarray::Array1;
use std::path::Path;

/// Mathematical formulae accepting a single scalar argument.
#[load]
pub enum Formula {
    /// Constant value. = c
    Constant(f64),
    /// Linear formula. = (x * m) + c
    Linear(f64, f64),
    /// Bifurcation formula. = x < y ? a : b.
    Bifurcation(f64, f64, f64),
    /// Linear interpolation between points.
    LinearInterpolation(Array1<f64>, Array1<f64>),
}

impl Build for Formula {
    type Inst = crate::Formula;

    #[inline]
    fn build(self, _in_dir: &Path) -> Result<Self::Inst, Error> {
        Ok(match self {
            Self::Constant(c) => Self::Inst::Constant { c },
            Self::Linear(c, m) => Self::Inst::Linear { c, m },
            Self::Bifurcation(t, under, over) => Self::Inst::Bifurcation { t, under, over },
            Self::LinearInterpolation(xs, ys) => Self::Inst::new_linear_interpolation(xs, ys),
        })
    }
}
