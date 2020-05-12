//! Adaptive grid settings implementation.

use crate::clone;
use attr::load;
use std::fmt::{Display, Formatter, Result};

/// Adaptive grid settings.
#[load]
pub struct Adaptive {
    /// Target maximum number of triangles per cell.
    tar_tris: usize,
    /// Maximum mesh depth.
    max_depth: i32,
    /// Collision detection padding.
    padding: f64,
}

impl Adaptive {
    clone!(tar_tris, usize);
    clone!(max_depth, i32);
    clone!(padding, f64);
}

impl Display for Adaptive {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        writeln!(fmt, "{:>32}  : {:?}", "tar_tris", self.tar_tris)?;
        writeln!(fmt, "{:>32}  : {:?}", "max_depth", self.max_depth)?;
        writeln!(fmt, "{:>32}  : {:?}", "padding", self.padding)
    }
}
