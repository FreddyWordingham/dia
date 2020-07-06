//! Camera module.

pub mod focus;
pub mod lens;
pub mod sensor;

pub use self::{focus::*, lens::*, sensor::*};

use crate::{access, display_field, display_field_ln};
use std::fmt::{Display, Formatter, Result};

/// Camera structure.
#[derive(Debug)]
pub struct Camera {
    /// Focus.
    focus: Focus,
    /// Lens.
    lens: Lens,
    /// Sensor.
    sensor: Sensor,
}

impl Camera {
    access!(focus, Focus);
    access!(lens, Lens);
    access!(sensor, Sensor);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(focus: Focus, lens: Lens, sensor: Sensor) -> Self {
        Self {
            focus,
            lens,
            sensor,
        }
    }
}

impl Display for Camera {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        display_field_ln!(fmt, "focus", &self.focus)?;
        display_field_ln!(fmt, "lens", &self.lens)?;
        display_field!(fmt, "sensor", &self.sensor)
    }
}
