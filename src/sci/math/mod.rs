//! Math module.

pub mod sample;

pub use self::sample::*;

use nalgebra::{Point3, Unit, Vector3};

/// Three-dimensional vector alias.
pub type Vec3 = Vector3<f64>;

/// Normalised three-dimensional vector alias.
pub type Dir3 = Unit<Vector3<f64>>;

/// Three-dimensional position alias.
pub type Pos3 = Point3<f64>;
