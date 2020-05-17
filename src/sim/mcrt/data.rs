//! Output data structure.

use crate::report;
use attr::save;
use std::{
    fmt::{Display, Formatter, Result},
    ops::AddAssign,
};

/// Output data structure.
#[save]
#[derive(Default)]
pub struct Data {
    /// Emitted photons.
    pub emitted_photons: f64,
}

impl AddAssign<&Self> for Data {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.emitted_photons += rhs.emitted_photons;
    }
}

impl Display for Data {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        write!(
            fmt,
            "{}",
            report::obj("emitted photons", self.emitted_photons).expect("Could not format field.")
        )
    }
}
