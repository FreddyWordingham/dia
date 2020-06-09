//! Output data structure.

use crate::{access, display_field, display_field_ln, Aabb, Error, Save, X, Y, Z};
use ndarray::Array3;
use std::{
    fmt::{Display, Formatter},
    ops::AddAssign,
    path::Path,
};

/// Output data structure.
pub struct Output {
    /// Measured volume.
    boundary: Aabb,
    /// Local total weight of emitted photons.
    pub emitted_photons: Array3<f64>,
    /// Dist travelled by photons [m].
    pub dist_travelled: Array3<f64>,
}

impl Output {
    access!(boundary, Aabb);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(boundary: Aabb, res: [usize; 3]) -> Self {
        debug_assert!(res[X] > 0);
        debug_assert!(res[Y] > 0);
        debug_assert!(res[Z] > 0);

        Self {
            boundary,
            emitted_photons: Array3::zeros(res),
            dist_travelled: Array3::zeros(res),
        }
    }
}

impl AddAssign<&Self> for Output {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.emitted_photons += &rhs.emitted_photons;
        self.dist_travelled += &rhs.dist_travelled;
    }
}

impl Display for Output {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        display_field_ln!(fmt, "total emitted photons", self.emitted_photons.sum())?;
        display_field!(
            fmt,
            "total distance travelled",
            self.dist_travelled.sum(),
            "m"
        )
    }
}

impl Save for Output {
    #[inline]
    fn save(&self, out_dir: &Path) -> Result<(), Error> {
        let path = out_dir.join("emitted_photon.nc");
        println!("saving: {}", path.display());
        self.emitted_photons.save(&path)?;

        let path = out_dir.join("dist_travelled.nc");
        println!("saving: {}", path.display());
        self.dist_travelled.save(&path)
    }
}
