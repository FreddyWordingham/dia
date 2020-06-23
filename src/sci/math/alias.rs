//! Aliases.

use nalgebra::{Point2, Point3, Rotation3, Similarity3, Unit, Vector2, Vector3};

/// Three-dimensional vector alias.
pub type Vec2 = Vector2<f64>;
pub type Vec3 = Vector3<f64>;

/// Normalised three-dimensional vector alias.
pub type Dir2 = Unit<Vector2<f64>>;
pub type Dir3 = Unit<Vector3<f64>>;

/// Three-dimensional position alias.
pub type Pos2 = Point2<f64>;
pub type Pos3 = Point3<f64>;

/// Three-dimensional rotation alias.
pub type Rot3 = Rotation3<f64>;

/// Three-dimensional transformation alias.
pub type Trans3 = Similarity3<f64>;
