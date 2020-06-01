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
    /// Line formula. = (x * m) + c
    Line(f64, f64),
    /// Bifurcation formula. = x < y ? a : b.
    Bifurcation(f64, f64, f64),
    /// Linear interpolation between points.
    Linear(Vec<f64>, Vec<f64>),
    /// Constant value spline.
    ConstantSpline(Vec<f64>, Vec<f64>),
}

impl Build for Formula {
    type Inst = crate::Formula;

    #[inline]
    fn build(self, _in_dir: &Path) -> Result<Self::Inst, Error> {
        Ok(match self {
            Self::Constant(c) => Self::Inst::Constant { c },
            Self::Line(c, m) => Self::Inst::Line { c, m },
            Self::Bifurcation(t, under, over) => Self::Inst::Bifurcation { t, under, over },
            Self::Linear(xs, ys) => Self::Inst::new_linear(Array1::from(xs), Array1::from(ys)),
            Self::ConstantSpline(xs, ys) => {
                Self::Inst::new_constant_spline(Array1::from(xs), Array1::from(ys))
            }
        })
    }
}
