//! Flat triangle implementation.

use crate::{
    access, Dir3,
    Greek::{Alpha, Beta, Gamma},
    Pos3,
};

/// Triangle.
pub struct Triangle {
    /// Vertex points.
    verts: [Pos3; 3],
    /// Surface plane normal.
    plane_norm: Dir3,
}

impl Triangle {
    access!(verts, [Pos3; 3]);
    access!(plane_norm, Dir3);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(verts: [Pos3; 3]) -> Self {
        let plane_norm = Self::init_plane_norm(&verts);

        Self { verts, plane_norm }
    }

    /// Initialise the plane normal.
    #[inline]
    #[must_use]
    fn init_plane_norm(verts: &[Pos3; 3]) -> Dir3 {
        Dir3::new_normalize(
            (verts.get(Alpha as usize).unwrap() - verts.get(Gamma as usize).unwrap())
                .cross(&(verts.get(Beta as usize).unwrap() - verts.get(Alpha as usize).unwrap())),
        )
    }

    /// Calculate the side lengths.
    #[inline]
    #[must_use]
    pub fn side_lengths(&self) -> [f64; 3] {
        let ab = nalgebra::distance(
            self.verts.get(Alpha as usize).unwrap(),
            self.verts.get(Beta as usize).unwrap(),
        );
        let bc = nalgebra::distance(
            self.verts.get(Beta as usize).unwrap(),
            self.verts.get(Gamma as usize).unwrap(),
        );
        let ca = nalgebra::distance(
            self.verts.get(Gamma as usize).unwrap(),
            self.verts.get(Alpha as usize).unwrap(),
        );

        [ab, bc, ca]
    }

    /// Calculate the perimeter length.
    #[inline]
    #[must_use]
    pub fn perimeter(&self) -> f64 {
        let [ab, bc, ca] = self.side_lengths();
        ab + bc + ca
    }

    /// Calculate the surface area.
    #[inline]
    #[must_use]
    pub fn area(&self) -> f64 {
        let [ab, bc, ca] = self.side_lengths();
        let s = (ab + bc + ca) * 0.5;
        (s * (s - ab) * (s - bc) * (s - ca)).sqrt()
    }

    /// Centre point.
    #[inline]
    #[must_use]
    pub fn centre(&self) -> Pos3 {
        Pos3::from(
            ((self.verts.get(Alpha as usize).unwrap().to_homogeneous()
                + self.verts.get(Beta as usize).unwrap().to_homogeneous()
                + self.verts.get(Gamma as usize).unwrap().to_homogeneous())
                / 3.0)
                .xyz(),
        )
    }
}
