//! Output data structure.

use crate::{report, Average, Error, Save, X, Y, Z};
use ndarray::Array3;
use std::{
    fmt::{Display, Formatter},
    ops::AddAssign,
    path::Path,
};

/// Output data structure.
pub struct Data {
    /// Local total weight of emitted photons.
    pub emitted_photons: Array3<f64>,
    /// Dist travelled by photons [m].
    pub dist_travelled: Array3<f64>,
    /// Local total weight of scattering events.
    pub scatters: Array3<f64>,
    /// Average rotations made by photons [rad].
    pub rotations: Array3<Average>,
    /// Local total weight of photon surface hits.
    pub hits: Array3<f64>,
}

impl Data {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(res: [usize; 3]) -> Self {
        debug_assert!(res[X] > 0);
        debug_assert!(res[Y] > 0);
        debug_assert!(res[Z] > 0);

        Self {
            emitted_photons: Array3::zeros(res),
            dist_travelled: Array3::zeros(res),
            scatters: Array3::zeros(res),
            rotations: Array3::default(res),
            hits: Array3::zeros(res),
        }
    }
}

impl AddAssign<&Self> for Data {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.emitted_photons += &rhs.emitted_photons;
        self.dist_travelled += &rhs.dist_travelled;
        self.scatters += &rhs.scatters;
        self.rotations += &rhs.rotations;
        self.hits += &rhs.hits;
    }
}

impl Display for Data {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        writeln!(
            fmt,
            "{}",
            report::obj("emitted photons sum", self.emitted_photons.sum())
                .expect("Could not format field.")
        )?;
        writeln!(
            fmt,
            "{}",
            report::obj_units("total distance travelled", self.dist_travelled.sum(), "m")
                .expect("Could not format field.")
        )?;
        writeln!(
            fmt,
            "{}",
            report::obj("total scatterings", self.scatters.sum()).expect("Could not format field.")
        )?;
        writeln!(
            fmt,
            "{}",
            report::obj_units(
                "Average rotation",
                self.rotations.map(|x| x.ave()).sum().to_degrees(),
                "deg"
            )
            .expect("Could not format field.")
        )?;
        write!(
            fmt,
            "{}",
            report::obj("total surface hits", self.hits.sum()).expect("Could not format field.")
        )
    }
}

impl Save for Data {
    #[inline]
    fn save(&self, out_dir: &Path) -> Result<(), Error> {
        let path = out_dir.join("emitted_photon.nc");
        println!("saving: {}", path.display());
        self.emitted_photons.save(&path)?;

        let path = out_dir.join("dist_travelled.nc");
        println!("saving: {}", path.display());
        self.dist_travelled.save(&path)?;

        let path = out_dir.join("scatters.nc");
        println!("saving: {}", path.display());
        self.scatters.save(&path)?;

        let path = out_dir.join("ave_rotations.nc");
        println!("saving: {}", path.display());
        self.rotations.map(|x| x.ave()).save(&path)?;

        let path = out_dir.join("hits.nc");
        println!("saving: {}", path.display());
        self.hits.save(&path)
    }
}
