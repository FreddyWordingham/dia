//! Adaptive grid settings implementation.

use crate::clone;
use attr::load;

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
