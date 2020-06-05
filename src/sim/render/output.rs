//! Input module.

use crate::{Error, Image, Save, X, Y, Z};
use ndarray::Array3;
use std::{ops::AddAssign, path::Path};

/// Render simulation output structure.
pub struct Output {
    /// Base image.
    pub image: Image,
    /// Number of hits.
    pub hits: Array3<f64>,
}

impl Output {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(grid_res: [usize; 3], img_res: [usize; 2]) -> Self {
        debug_assert!(grid_res[X] > 0);
        debug_assert!(grid_res[Y] > 0);
        debug_assert!(grid_res[Z] > 0);
        debug_assert!(img_res[X] > 0);
        debug_assert!(img_res[Y] > 0);

        Self {
            image: Image::default(img_res),
            hits: Array3::zeros(grid_res),
        }
    }
}

impl AddAssign<&Self> for Output {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.image += &rhs.image;
        self.hits += &rhs.hits;
    }
}

impl Save for Output {
    #[inline]
    fn save(&self, out_dir: &Path) -> Result<(), Error> {
        let path = out_dir.join("image.png");
        println!("Saving: {}", path.display());
        self.image.save(&path)?;
        let path = out_dir.join("hits.nc");
        println!("Saving: {}", path.display());
        self.hits.save(&path)
    }
}
