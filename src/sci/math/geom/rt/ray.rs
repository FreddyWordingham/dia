//! Ray implementation.

use crate::{access, Dir3, Pos3};

/// Ray structure.
#[derive(Clone)]
pub struct Ray {
    /// Ray origin.
    pos: Pos3,
    /// Ray direction.
    dir: Dir3,
}

impl Ray {
    access!(pos, Pos3);
    access!(dir, dir_mut, Dir3);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(pos: Pos3, dir: Dir3) -> Self {
        Self { pos, dir }
    }

    /// Destruct self into components.
    #[inline]
    #[must_use]
    pub const fn destruct(self) -> (Pos3, Dir3) {
        (self.pos, self.dir)
    }

    /// Move along the direction of travel a given distance.
    #[inline]
    pub fn travel(&mut self, dist: f64) {
        debug_assert!(dist > 0.0);

        self.pos += self.dir.as_ref() * dist;
    }
}
