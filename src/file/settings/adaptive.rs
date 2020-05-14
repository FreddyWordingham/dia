//! Adaptive grid settings implementation.

use crate::{clone, report};
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
        writeln!(
            fmt,
            "{}",
            report::obj("target triangles", self.tar_tris).expect("Could not format field.")
        )?;
        writeln!(
            fmt,
            "{}",
            report::obj("maximum depth", self.max_depth).expect("Could not format field.")
        )?;
        write!(
            fmt,
            "{}",
            report::obj_units("padding", self.padding * 100.0, "%")
                .expect("Could not format field.")
        )
    }
}
