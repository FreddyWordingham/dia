//! Scene implementation.

use crate::{access, Aabb, Error, Group, Load, Mesh, Set};
use std::path::Path;

/// Scene collection.
pub struct Scene {
    /// Boundary.
    boundary: Aabb,
    /// Surfaces forming the scene.
    surfs: Set<Vec<Mesh>>,
}

impl Scene {
    access!(boundary, Aabb);
    access!(surfs, Set<Vec<Mesh>>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(surfs: Set<Vec<Mesh>>) -> Self {
        let boundary = Self::init_boundary(&surfs);
        Self { boundary, surfs }
    }

    /// Initialise the boundary encompassing all of the mesh vertices.
    #[inline]
    #[must_use]
    fn init_boundary(surfs: &Set<Vec<Mesh>>) -> Aabb {
        let mut mins = None;
        let mut maxs = None;

        for meshes in surfs.values() {
            for mesh in meshes {
                let (mesh_mins, mesh_maxs) = mesh.boundary().mins_maxs();

                if mins.is_none() {
                    mins = Some(mesh_mins);
                } else {
                    for (grid_min, mesh_min) in
                        mins.as_mut().unwrap().iter_mut().zip(mesh_mins.iter())
                    {
                        if mesh_min < grid_min {
                            *grid_min = *mesh_min;
                        }
                    }
                }

                if maxs.is_none() {
                    maxs = Some(mesh_maxs);
                } else {
                    for (grid_max, mesh_max) in
                        maxs.as_mut().unwrap().iter_mut().zip(mesh_maxs.iter())
                    {
                        if mesh_max > grid_max {
                            *grid_max = *mesh_max;
                        }
                    }
                }
            }
        }

        Aabb::new(mins.unwrap(), maxs.unwrap())
    }

    /// Load in a set of meshes.
    #[inline]
    #[must_use]
    pub fn load<'a, I>(in_dir: &Path, names: I) -> Result<Self, Error>
    where
        I: Iterator<Item = (&'a Group, &'a Vec<String>)>,
    {
        let mut surfs: Set<Vec<Mesh>> = Set::new();
        for (group, meshes) in names {
            for mesh in meshes {
                let path = in_dir.join(format!("{}.obj", mesh));
                println!("Loading mesh: {}", path.display());
                let mesh: Mesh = Mesh::load(&path)?;

                if let Some(entry) = surfs.get_mut(group) {
                    entry.push(mesh);
                } else {
                    surfs.insert(*group, vec![mesh]);
                }
            }
        }

        Ok(Self::new(surfs))
    }
}
