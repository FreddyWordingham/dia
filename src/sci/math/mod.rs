//! Math module.

pub mod lambda;
pub mod rng;
pub mod sample;
pub mod sort;

pub use self::lambda::*;
pub use self::rng::*;
pub use self::sample::*;
pub use self::sort::*;

use nalgebra::{Point3, Unit, Vector3};

/// Three-dimensional vector alias.
pub type Vec3 = Vector3<f64>;

/// Normalised three-dimensional vector alias.
pub type Dir3 = Unit<Vector3<f64>>;

/// Three-dimensional position alias.
pub type Pos3 = Point3<f64>;
