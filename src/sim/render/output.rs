//! Input module.

use crate::{Error, Image, Save, X, Y};
use std::{ops::AddAssign, path::Path};

/// Render simulation output structure.
pub struct Output {
    /// Base image.
    pub image: Image,
}

impl Output {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(img_res: [usize; 2]) -> Self {
        debug_assert!(img_res[X] > 0);
        debug_assert!(img_res[Y] > 0);

        Self {
            image: Image::default(img_res),
        }
    }
}

impl AddAssign<&Self> for Output {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.image += &rhs.image;
    }
}

impl Save for Output {
    #[inline]
    fn save(&self, out_dir: &Path) -> Result<(), Error> {
        let path = out_dir.join("image.png");
        println!("Saving: {}", path.display());
        self.image.save(&path)
    }
}
