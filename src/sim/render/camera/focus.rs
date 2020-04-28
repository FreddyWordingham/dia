//! Focus implementation.

use crate::{access, Dir3, Orient, Pos3, Ray};

/// Focus structure.
pub struct Focus {
    /// Orientation.
    orient: Orient,
    /// Target point.
    tar: Pos3,
}

impl Focus {
    access!(orient, Orient);
    access!(tar, Pos3);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(pos: Pos3, tar: Pos3) -> Self {
        Self {
            orient: Orient::new(Ray::new(pos, Dir3::new_normalize(tar - pos))),
            tar,
        }
    }
}
