//! Smooth triangle-mesh implementation.

use crate::{access, Aabb, Cartesian::X, Greek::Alpha, SmoothTriangle, Trans3, Transform};

/// Mesh geometry.
pub struct Mesh {
    /// Bounding box.
    aabb: Aabb,
    /// List of component triangles.
    tris: Vec<SmoothTriangle>,
}

impl Mesh {
    access!(aabb, Aabb);
    access!(tris, Vec<SmoothTriangle>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(tris: Vec<SmoothTriangle>) -> Self {
        Self {
            aabb: Self::init_aabb(&tris),
            tris,
        }
    }

    /// Initialise the bounding box for the mesh.
    fn init_aabb(tris: &[SmoothTriangle]) -> Aabb {
        let mut mins = *tris
            .get(X as usize)
            .unwrap()
            .tri()
            .verts()
            .get(Alpha as usize)
            .unwrap();
        let mut maxs = mins;

        for tri in tris {
            for v in tri.tri().verts().iter() {
                for (v, (min, max)) in v.iter().zip(mins.iter_mut().zip(maxs.iter_mut())) {
                    if *min > *v {
                        *min = *v;
                    } else if *max < *v {
                        *max = *v;
                    }
                }
            }
        }

        Aabb::new(mins, maxs)
    }

    /// Calculate the total surface area.
    #[inline]
    #[must_use]
    pub fn area(&self) -> f64 {
        let mut area = 0.0;

        for tri in &self.tris {
            area += tri.tri().area();
        }

        area
    }
}

impl Transform for Mesh {
    #[inline]
    fn transform(&mut self, trans: &Trans3) {
        for tri in &mut self.tris {
            tri.transform(trans);
        }

        self.aabb = Self::init_aabb(&self.tris);
    }
}
