//! Adaptive gridding scheme.

use crate::{Aabb, Group, SmoothTriangle};

/// Adaptive cell enumeration.
///
///         6npp   7ppp
///       4nnp   5pnp
/// z  y    2npn   3ppn
/// | /   0nnn   1pnn
/// |/__x
pub enum Adaptive<'a> {
    /// Root cell.
    Root {
        /// Boundary.
        boundary: Aabb,
        /// Children.
        children: [Box<Adaptive<'a>>; 8],
    },
    /// Branching cell.
    Branch {
        /// Boundary.
        boundary: Aabb,
        /// Children.
        children: [Box<Adaptive<'a>>; 8],
    },
    /// Terminal populated cell.
    Leaf {
        /// Boundary.
        boundary: Aabb,
        /// Intersecting triangles.
        tris: Vec<(Group, &'a SmoothTriangle)>,
    },
    /// Terminal empty cell.
    Empty {
        /// Boundary.
        boundary: Aabb,
    },
}

pub mod constructor;
pub mod properties;

pub use self::constructor::*;
pub use self::properties::*;
