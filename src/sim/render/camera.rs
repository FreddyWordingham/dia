//! Camera implementation.

use crate::Pos3;

/// Camera structure.
pub struct Camera {
    /// Position.
    pos: Pos3,
    /// Target point.
    tar: Pos3,
}

impl Camera {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(pos: Pos3, tar: Pos3) -> Self {
        Self { pos, tar }
    }
}
