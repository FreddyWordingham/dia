//! Output data structure.

use crate::report;
use attr::save;
use ndarray::Array3;
use std::{
    fmt::{Display, Formatter, Result},
    ops::AddAssign,
};

/// Output data structure.
#[save]
pub struct Data {
    /// Emitted photons.
    pub emitted_photons: Array3<f64>,
}

impl Data {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(res: [usize; 3]) -> Self {
        Self {
            emitted_photons: Array3::zeros(res),
        }
    }
}

impl AddAssign<&Self> for Data {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.emitted_photons += &rhs.emitted_photons;
    }
}

impl Display for Data {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        write!(
            fmt,
            "{}",
            report::obj("emitted photons sum", self.emitted_photons.sum())
                .expect("Could not format field.")
        )
    }
}
