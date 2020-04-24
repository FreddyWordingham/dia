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

impl<'a> Adaptive<'a> {
    // /// Construct a new adaptive grid.
    // #[inline]
    // #[must_use]
    // pub fn new_root(settings: &GridSettings, scene: &'a Scene) -> Self {
    //     let mut boundary = scene.boundary().clone();
    //     boundary.expand(settings.padding());

    //     let mut tris = Vec::new();
    //     for (group, meshes) in scene.groups() {
    //         for mesh in meshes {
    //             tris.reserve(mesh.tris().len());
    //             for tri in mesh.tris() {
    //                 tris.push((*group, tri));
    //             }
    //         }
    //     }

    //     let children = Self::init_children(settings, &boundary, 1, &tris);

    //     Self::Root { boundary, children }
    // }
}
