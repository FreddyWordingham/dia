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
    /// Constant value spline.
    ConstantSpline {
        /// X change points.
        xs: Array1<f64>,
        /// Y values.
        ys: Array1<f64>,
    },
    /// Linear spline.
    LinearSpline {
        /// X change points.
        xs: Array1<f64>,
        /// Y values.
        ys: Array1<f64>,
        /// Gradient between points.
        grads: Array1<f64>,
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

    /// Construct a constant spline instance.
    #[inline]
    #[must_use]
    pub fn new_constant_spline(xs: Array1<f64>, ys: Array1<f64>) -> Self {
        debug_assert!(xs.len() >= 2);
        debug_assert!(xs.len() == (ys.len() + 1));
        debug_assert!(order::is_ascending(xs.as_slice().unwrap()));

        Self::ConstantSpline { xs, ys }
    }

    /// Construct a linear spline instance.
    #[inline]
    #[must_use]
    pub fn new_linear_spline(xs: Array1<f64>, ys: Array1<f64>, grads: Array1<f64>) -> Self {
        debug_assert!(xs.len() >= 2);
        debug_assert!(xs.len() == ys.len());
        debug_assert!(xs.len() == (grads.len() + 1));
        debug_assert!(order::is_ascending(xs.as_slice().unwrap()));

        Self::LinearSpline { xs, ys, grads }
    }

    /// Construct a linear spline instance.
    #[inline]
    #[must_use]
    pub fn new_linear_spline_auto(xs: Array1<f64>, ys: Array1<f64>) -> Self {
        debug_assert!(xs.len() >= 2);
        debug_assert!(xs.len() == ys.len());
        debug_assert!(order::is_ascending(xs.as_slice().unwrap()));

        let mut grads = Vec::with_capacity(xs.len() - 1);
        for ((x_curr, x_next), (y_curr, y_next)) in xs
            .iter()
            .zip(xs.iter().skip(1))
            .zip(ys.iter().zip(ys.iter().skip(1)))
        {
            grads.push((y_next - y_curr) / (x_next - x_curr));
        }

        Self::new_linear_spline(xs, ys, Array1::from(grads))
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
            Self::ConstantSpline { xs, ys } => {
                debug_assert!(x >= xs[0]);
                debug_assert!(x <= xs[xs.len() - 1]);

                for (index, xn) in xs.iter().enumerate() {
                    if *xn > x {
                        return ys[index - 1];
                    }
                }
                return ys[ys.len() - 1];
            }
            Self::LinearSpline { xs, ys, grads } => {
                debug_assert!(x >= xs[0]);
                debug_assert!(x <= xs[xs.len() - 1]);

                for (index, xn) in xs.iter().enumerate() {
                    if *xn > x {
                        let dx = x - xs[index - 1];
                        return ys[index - 1] + (grads[index - 1] * dx);
                    }
                }
                return ys[ys.len() - 1];
            }
        }
    }
}
