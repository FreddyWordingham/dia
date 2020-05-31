//! Formula implementation.

use crate::{order, Range};
use ndarray::Array1;

/// Mathematical formulae accepting a single scalar argument.
#[derive(Debug)]
pub enum Formula {
    /// Constant value. = c
    Constant {
        /// Constant.
        c: f64,
    },
    /// Line formula. = mx + c
    Line {
        /// Offset.
        c: f64,
        /// Gradient.
        m: f64,
    },
    /// Bifurcation formula. = if x < t { under } else { over }.
    Bifurcation {
        /// Threshold value.
        t: f64,
        /// Under value.
        under: f64,
        /// Over value.
        over: f64,
    },
    /// Linear interpolation between points.
    Linear {
        /// X points.
        xs: Array1<f64>,
        /// Y points.
        ys: Array1<f64>,
        /// Gradient between points.
        grads: Array1<f64>,
        /// Valid domain range.
        range: Range,
    },
}

impl Formula {
    /// Construct a new linear interpolation instance.
    #[inline]
    #[must_use]
    pub fn new_linear(xs: Array1<f64>, ys: Array1<f64>) -> Self {
        debug_assert!(xs.len() >= 2);
        debug_assert!(xs.len() == ys.len());
        debug_assert!(order::is_ascending(xs.as_slice().unwrap()));

        let mut grads = Vec::with_capacity(xs.len() - 1);
        for (curr_x, (next_x, (curr_y, next_y))) in xs
            .iter()
            .zip(xs.iter().skip(1).zip(ys.iter().zip(ys.iter().skip(1))))
        {
            let delta_x = next_x - curr_x;
            let delta_y = next_y - curr_y;

            grads.push(delta_y / delta_x);
        }

        let range = Range::new(xs[0], xs[xs.len() - 1]);

        Self::Linear {
            xs,
            ys,
            grads: Array1::from(grads),
            range,
        }
    }

    /// Determine the corresponding output value for the given input.
    #[inline]
    #[must_use]
    pub fn y(&self, x: f64) -> f64 {
        match self {
            Self::Constant { c } => *c,
            Self::Line { c, m } => (x * m) + c,
            Self::Bifurcation { t, under, over } => {
                if x < *t {
                    *under
                } else {
                    *over
                }
            }
            Self::Linear {
                xs,
                ys,
                grads,
                range,
            } => {
                debug_assert!(range.contains(x));

                for (xn, (yn, grad)) in xs.iter().skip(1).zip(ys.iter().skip(1).zip(grads.iter())) {
                    if x <= *xn {
                        let delta = x - *xn;
                        return *yn + (delta * grad);
                    }
                }

                unreachable!();
            }
        }
    }
}
