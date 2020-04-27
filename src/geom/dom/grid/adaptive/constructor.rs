//! Constructor methods.

use crate::{
    settings::Adaptive as Settings, Aabb, Collide, Group, Mesh, Pos3, Scene, Set, SmoothTriangle,
};

impl<'a> super::Adaptive<'a> {
    /// Construct a new adaptive grid.
    #[inline]
    #[must_use]
    pub fn new_root(settings: &Settings, scene: &'a Scene) -> Self {
        let mut boundary = scene.boundary().clone();
        boundary.expand(settings.padding());

        let mut tris = Vec::new();
        for (group, meshes) in scene.surfs() {
            for mesh in meshes {
                tris.reserve(mesh.tris().len());
                for tri in mesh.tris() {
                    tris.push((*group, tri));
                }
            }
        }
        let children = Self::init_children(settings, &boundary, 0, &tris);

        Self::Root { boundary, children }
    }

    /// Initialise a child cell.
    #[inline]
    #[must_use]
    fn init_child(
        settings: &Settings,
        boundary: Aabb,
        depth: i32,
        potential_tris: &[(Group, &'a SmoothTriangle)],
    ) -> Self {
        debug_assert!(depth <= settings.max_depth());

        let mut detection_vol = boundary.clone();
        detection_vol.expand(settings.padding());

        let mut tris = Vec::new();
        for (group, tri) in potential_tris {
            if tri.overlap(&detection_vol) {
                tris.push((*group, *tri));
            }
        }

        if tris.is_empty() {
            return Self::Empty { boundary };
        }

        if (tris.len() <= settings.tar_tris()) || (depth >= settings.max_depth()) {
            return Self::Leaf { boundary, tris };
        }

        let children = Self::init_children(settings, &boundary, depth + 1, &tris);

        Self::Branch { boundary, children }
    }

    /// Initialise the children of a branching cell.
    #[allow(clippy::similar_names)]
    #[inline]
    #[must_use]
    fn init_children(
        settings: &Settings,
        parent_boundary: &Aabb,
        depth: i32,
        potential_tris: &[(Group, &'a SmoothTriangle)],
    ) -> [Box<Self>; 8] {
        debug_assert!(depth <= settings.max_depth());
        debug_assert!(!potential_tris.is_empty());

        let hws = parent_boundary.half_widths();
        let make_child = |min_x: f64, min_y: f64, min_z: f64| {
            let min = Pos3::new(min_x, min_y, min_z);
            Box::new(Self::init_child(
                settings,
                Aabb::new(min, min + hws),
                depth,
                potential_tris,
            ))
        };

        let mins = parent_boundary.mins();
        let min_x = mins.x;
        let min_y = mins.y;
        let min_z = mins.z;

        let nnn = make_child(min_x, min_y, min_z);
        let pnn = make_child(min_x + hws.x, min_y, min_z);
        let npn = make_child(min_x, min_y + hws.y, min_z);
        let ppn = make_child(min_x + hws.x, min_y + hws.y, min_z);
        let nnp = make_child(min_x, min_y, min_z + hws.z);
        let pnp = make_child(min_x + hws.x, min_y, min_z + hws.z);
        let npp = make_child(min_x, min_y + hws.y, min_z + hws.z);
        let ppp = make_child(min_x + hws.x, min_y + hws.y, min_z + hws.z);

        [nnn, pnn, npn, ppn, nnp, pnp, npp, ppp]
    }
}
