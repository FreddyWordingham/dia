//! Aliases.

use nalgebra::{Point3, Rotation3, Similarity3, Unit, Vector3};

/// Three-dimensional vector alias.
pub type Vec3 = Vector3<f64>;

/// Normalised three-dimensional vector alias.
pub type Dir3 = Unit<Vector3<f64>>;

/// Three-dimensional position alias.
pub type Pos3 = Point3<f64>;

/// Three-dimensional rotation alias.
pub type Rot3 = Rotation3<f64>;

/// Three-dimensional transformation alias.
pub type Trans3 = Similarity3<f64>;
