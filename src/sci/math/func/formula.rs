//! Formula implementation.

/// Mathematical formulae accepting a single scalar argument.
#[derive(Debug)]
pub enum Formula {
    /// Constant value. = c
    Constant {
        /// Constant.
        c: f64,
    },
    /// Linear formula. = mx + c
    Linear {
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
}

impl Formula {
    /// Determine the corresponding output value for the given input.
    #[inline]
    #[must_use]
    pub fn y(&self, x: f64) -> f64 {
        match self {
            Self::Constant { c } => *c,
            Self::Linear { c, m } => (x * m) + c,
            Self::Bifurcation { t, under, over } => {
                if x < *t {
                    *under
                } else {
                    *over
                }
            }
        }
    }
}
