//! Camera module.

pub mod focus;
pub mod lens;
pub mod sensor;

pub use self::{focus::*, lens::*, sensor::*};

use crate::access;

/// Camera structure.
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
    pub fn new(focus: Focus, lens: Lens, sensor: Sensor) -> Self {
        Self {
            focus,
            lens,
            sensor,
        }
    }
}
