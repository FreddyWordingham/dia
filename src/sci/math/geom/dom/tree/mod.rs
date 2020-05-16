//! Adaptive tree cell scheme.

use crate::{Aabb, Group, SmoothTriangle};

/// Tree cell enumeration.
///
///         6npp   7ppp
///       4nnp   5pnp
/// z  y    2npn   3ppn
/// | /   0nnn   1pnn
/// |/__x
pub enum Cell<'a> {
    /// Root cell.
    Root {
        /// Boundary.
        boundary: Aabb,
        /// Children.
        children: [Box<Cell<'a>>; 8],
    },
    /// Branching cell.
    Branch {
        /// Boundary.
        boundary: Aabb,
        /// Children.
        children: [Box<Cell<'a>>; 8],
    },
    /// Terminal populated cell.
    Leaf {
        /// Boundary.
        boundary: Aabb,
        /// Intersecting triangles.
        tris: Vec<(&'a Group, &'a SmoothTriangle)>,
    },
    /// Terminal empty cell.
    Empty {
        /// Boundary.
        boundary: Aabb,
    },
}

impl<'a> Cell<'a> {
    /// Reference the cell's boundary.
    #[inline]
    #[must_use]
    pub fn boundary(&self) -> &Aabb {
        match self {
            Self::Root { boundary, .. }
            | Self::Branch { boundary, .. }
            | Self::Leaf { boundary, .. }
            | Self::Empty { boundary, .. } => boundary,
        }
    }
}

pub mod construct;
pub mod settings;
// pub mod properties;
// pub mod scan;
// pub mod search;

// pub use self::{construct::*, properties::*, scan::*, search::*};
pub use self::construct::*;
pub use self::settings::*;