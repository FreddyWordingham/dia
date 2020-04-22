//! Smooth triangle-mesh implementation.

use crate::{
    access, Aabb, Collide, Dir3, Error, Load, Pos3, Ray, Side, SmoothTriangle, Trace, Trans3,
    Transform, Vec3, ALPHA, X,
};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    result::Result,
};

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
        let mut mins = tris[X].tri().verts()[ALPHA];
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

impl Collide for Mesh {
    #[inline]
    #[must_use]
    fn overlap(&self, aabb: &Aabb) -> bool {
        if !self.aabb.overlap(aabb) {
            return false;
        }

        for tri in &self.tris {
            if tri.overlap(aabb) {
                return true;
            }
        }

        false
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

impl Trace for Mesh {
    #[inline]
    #[must_use]
    fn hit(&self, ray: &Ray) -> bool {
        if !self.aabb.hit(ray) {
            return false;
        }

        self.tris.iter().any(|t| t.hit(ray))
    }

    #[inline]
    #[must_use]
    fn dist(&self, ray: &Ray) -> Option<f64> {
        if !self.aabb.hit(ray) {
            return None;
        }

        self.tris
            .iter()
            .filter_map(|tri| tri.dist(ray))
            .min_by(|a, b| a.partial_cmp(b).unwrap())
    }

    #[inline]
    #[must_use]
    fn dist_side(&self, ray: &Ray) -> Option<(f64, Side)> {
        if !self.aabb.hit(ray) {
            return None;
        }

        self.tris
            .iter()
            .filter_map(|tri| tri.dist_side(ray))
            .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
    }
}

impl Load for Mesh {
    #[inline]
    fn load(path: &Path) -> Result<Self, Error> {
        let vertex_lines: Vec<_> = BufReader::new(File::open(path)?)
            .lines()
            .map(Result::unwrap)
            .filter(|line| line.starts_with("v "))
            .collect();

        let mut verts = Vec::with_capacity(vertex_lines.len());
        for line in vertex_lines {
            let mut words = line.split_whitespace();
            words.next();

            let px = words.next().unwrap().parse::<f64>()?;
            let py = words.next().unwrap().parse::<f64>()?;
            let pz = words.next().unwrap().parse::<f64>()?;

            verts.push(Pos3::new(px, py, pz));
        }

        let normal_lines: Vec<_> = BufReader::new(File::open(path)?)
            .lines()
            .map(Result::unwrap)
            .filter(|line| line.starts_with("vn "))
            .collect();

        let mut norms = Vec::with_capacity(normal_lines.len());
        for line in normal_lines {
            let mut words = line.split_whitespace();
            words.next();

            let nx = words.next().unwrap().parse::<f64>()?;
            let ny = words.next().unwrap().parse::<f64>()?;
            let nz = words.next().unwrap().parse::<f64>()?;

            norms.push(Dir3::new_normalize(Vec3::new(nx, ny, nz)));
        }

        let face_lines: Vec<_> = BufReader::new(File::open(path)?)
            .lines()
            .map(Result::unwrap)
            .filter(|line| line.starts_with("f "))
            .collect();

        let mut faces = Vec::with_capacity(face_lines.len());
        for line in face_lines {
            let line = line.replace("//", " ");
            let mut words = line.split_whitespace();
            words.next();

            let fx = words.next().unwrap().parse::<usize>()? - 1;
            let nx = words.next().unwrap().parse::<usize>()? - 1;
            let fy = words.next().unwrap().parse::<usize>()? - 1;
            let ny = words.next().unwrap().parse::<usize>()? - 1;
            let fz = words.next().unwrap().parse::<usize>()? - 1;
            let nz = words.next().unwrap().parse::<usize>()? - 1;

            faces.push(((fx, fy, fz), (nx, ny, nz)));
        }

        let mut tris = Vec::with_capacity(faces.len());
        for face in faces {
            tris.push(SmoothTriangle::new_from_verts(
                [verts[(face.0).0], verts[(face.0).1], verts[(face.0).2]],
                [norms[(face.1).0], norms[(face.1).1], norms[(face.1).2]],
            ));
        }

        Ok(Self::new(tris))
    }
}
