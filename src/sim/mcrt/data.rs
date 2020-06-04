//! Output data structure.

use crate::{display_field, display_field_ln, Aabb, Average, Error, Histogram, Save, X, Y, Z};
use ndarray::Array3;
use std::{
    fmt::{Display, Formatter},
    ops::AddAssign,
    path::Path,
};

/// Output data structure.
pub struct Data {
    /// Measured volume.
    boundary: Aabb,
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
    /// Local photo-energy.
    pub energy: Array3<f64>,
    /// Local absorbed photo-energy.
    pub absorptions: Array3<f64>,
    /// Local shifted photo-energy.
    pub shifts: Array3<f64>,
    /// Recording spectrum 0.
    pub spec_0: Histogram,
    /// Recording spectrum 1.
    pub spec_1: Histogram,
    /// Recording spectrum 2.
    pub spec_2: Histogram,
    /// Recording spectrum 3.
    pub spec_3: Histogram,
    /// Recording spectrum 4.
    pub spec_4: Histogram,
}

impl Data {
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
            scatters: Array3::zeros(res),
            rotations: Array3::default(res),
            hits: Array3::zeros(res),
            energy: Array3::zeros(res),
            absorptions: Array3::zeros(res),
            shifts: Array3::zeros(res),
            spec_0: Histogram::new(300.0e-9, 800e-9, 500),
            spec_1: Histogram::new(300.0e-9, 800e-9, 500),
            spec_2: Histogram::new(300.0e-9, 800e-9, 500),
            spec_3: Histogram::new(300.0e-9, 800e-9, 500),
            spec_4: Histogram::new(300.0e-9, 800e-9, 500),
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
        self.energy += &rhs.energy;
        self.absorptions += &rhs.absorptions;
        self.shifts += &rhs.shifts;
        self.spec_0 += &rhs.spec_0;
        self.spec_1 += &rhs.spec_1;
        self.spec_2 += &rhs.spec_2;
        self.spec_3 += &rhs.spec_3;
        self.spec_4 += &rhs.spec_4;
    }
}

impl Display for Data {
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
        display_field_ln!(fmt, "total scatters", self.scatters.sum())?;
        display_field_ln!(
            fmt,
            "average rotation",
            self.rotations
                .map(Average::ave)
                .mean()
                .unwrap()
                .to_degrees(),
            "deg"
        )?;
        display_field_ln!(fmt, "total hits", self.hits.sum())?;
        display_field_ln!(fmt, "total energy", self.energy.sum(), "J")?;
        display_field_ln!(fmt, "total absorptions", self.absorptions.sum(), "J")?;
        display_field_ln!(fmt, "total shifts", self.shifts.sum())?;
        display_field_ln!(fmt, "histogram 0 total", self.spec_0.counts().sum())?;
        display_field_ln!(fmt, "histogram 1 total", self.spec_1.counts().sum())?;
        display_field_ln!(fmt, "histogram 2 total", self.spec_2.counts().sum())?;
        display_field_ln!(fmt, "histogram 3 total", self.spec_3.counts().sum())?;
        display_field!(fmt, "histogram 4 total", self.spec_4.counts().sum())
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
        self.rotations.map(Average::ave).save(&path)?;

        let path = out_dir.join("hits.nc");
        println!("saving: {}", path.display());
        self.hits.save(&path)?;

        let path = out_dir.join("energy_density.nc");
        println!("saving: {}", path.display());
        let cell_vol = self.boundary.vol() / self.energy.len() as f64;
        let energy_dens = self.energy.clone() / cell_vol;
        energy_dens.save(&path)?;

        let path = out_dir.join("absorption_density.nc");
        println!("saving: {}", path.display());
        let absorption_dens = self.absorptions.clone() / cell_vol;
        absorption_dens.save(&path)?;

        let path = out_dir.join("shift_density.nc");
        println!("saving: {}", path.display());
        let shift_dens = self.shifts.clone() / cell_vol;
        shift_dens.save(&path)?;

        let path = out_dir.join("spec_0.csv");
        println!("saving: {}", path.display());
        self.spec_0.save(&path)?;

        let path = out_dir.join("spec_1.csv");
        println!("saving: {}", path.display());
        self.spec_1.save(&path)?;

        let path = out_dir.join("spec_2.csv");
        println!("saving: {}", path.display());
        self.spec_2.save(&path)?;

        let path = out_dir.join("spec_3.csv");
        println!("saving: {}", path.display());
        self.spec_3.save(&path)?;

        let path = out_dir.join("spec_4.csv");
        println!("saving: {}", path.display());
        self.spec_4.save(&path)
    }
}
