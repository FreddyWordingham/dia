//! Output data structure.

use crate::{report, Error, Save};
use ndarray::Array3;
use std::{
    fmt::{Display, Formatter},
    ops::AddAssign,
    path::Path,
};

/// Output data structure.
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
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        write!(
            fmt,
            "{}",
            report::obj("emitted photons sum", self.emitted_photons.sum())
                .expect("Could not format field.")
        )
    }
}

impl Save for Data {
    #[inline]
    fn save(&self, out_dir: &Path) -> Result<(), Error> {
        let path = out_dir.join("emitted_photon.nc");
        println!("saving: {}", path.display());
        self.emitted_photons.save(&path)
    }
}
