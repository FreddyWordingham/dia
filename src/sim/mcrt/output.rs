//! Output data structure.

use crate::{
    access, clone, display_field, display_field_ln, Aabb, Error, Histogram, Pos3, Save, X, Y, Z,
};
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
    /// Cell volume [m^3].
    cell_vol: f64,
    /// Local total weight of emitted photons.
    pub emitted_photons: Array3<f64>,
    /// Dist travelled by photons [m].
    pub dist_travelled: Array3<f64>,
    /// Local energy [J].
    pub energy: Array3<f64>,
    /// Local absorptions [J].
    pub absorptions: Array3<f64>,
    /// Local shifts [J].
    pub shifts: Array3<f64>,
    /// Tracked paths.
    pub paths: Vec<Vec<Pos3>>,
    /// Spectrometer.
    pub spec: Histogram,
}

impl Output {
    access!(boundary, Aabb);
    clone!(cell_vol, f64);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(boundary: Aabb, res: [usize; 3]) -> Self {
        debug_assert!(res[X] > 0);
        debug_assert!(res[Y] > 0);
        debug_assert!(res[Z] > 0);

        let cell_vol = boundary.vol() / (res[X] * res[Y] * res[Z]) as f64;

        Self {
            boundary,
            cell_vol,
            emitted_photons: Array3::zeros(res),
            dist_travelled: Array3::zeros(res),
            energy: Array3::zeros(res),
            absorptions: Array3::zeros(res),
            shifts: Array3::zeros(res),
            paths: Vec::new(),
            spec: Histogram::new(0e-9, 1000e-9, 100),
        }
    }
}

impl AddAssign<Self> for Output {
    #[inline]
    fn add_assign(&mut self, mut rhs: Self) {
        self.emitted_photons += &rhs.emitted_photons;
        self.dist_travelled += &rhs.dist_travelled;
        self.energy += &rhs.energy;
        self.absorptions += &rhs.absorptions;
        self.shifts += &rhs.shifts;
        self.paths.append(&mut rhs.paths);
    }
}

impl Display for Output {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        display_field_ln!(fmt, "total emitted photons", self.emitted_photons.sum())?;
        display_field_ln!(
            fmt,
            "total distance travelled",
            self.dist_travelled.sum(),
            "m"
        )?;
        display_field_ln!(fmt, "total energy", self.energy.sum(), "J")?;
        display_field_ln!(fmt, "total absorption energy", self.absorptions.sum(), "J")?;
        display_field_ln!(fmt, "total shifted energy", self.shifts.sum(), "J")?;
        display_field!(fmt, "number of recorded paths", self.paths.len())
    }
}

impl Save for Output {
    #[inline]
    fn save(&self, out_dir: &Path) -> Result<(), Error> {
        // let path = out_dir.join("emission_dens.nc");
        // println!("saving: {}", path.display());
        // let emission_dens = &self.emitted_photons / self.cell_vol;
        // emission_dens.save(&path)?;

        // let path = out_dir.join("energy_dens.nc");
        // println!("saving: {}", path.display());
        // let energy_dens = &self.energy / self.cell_vol;
        // energy_dens.save(&path)?;

        // let path = out_dir.join("absorption_dens.nc");
        // println!("saving: {}", path.display());
        // let absorption_dens = &self.absorptions / self.cell_vol;
        // absorption_dens.save(&path)?;

        // let path = out_dir.join("shifted_dens.nc");
        // println!("saving: {}", path.display());
        // let shifted_dens = &self.shifts / self.cell_vol;
        // shifted_dens.save(&path)?;

        // let path = out_dir.join("dist_travelled.nc");
        // println!("saving: {}", path.display());
        // self.dist_travelled.save(&path)?;

        // let path = out_dir.join("spectrometer.csv");
        // println!("saving: {}", path.display());
        // self.spec.save(&path)

        Ok(())
    }
}
