//! Regular grid cell scheme.

pub mod settings;

pub use self::settings::*;

use crate::{access, report, Aabb, Error, Pos3, Vec3, X, Y, Z};
use std::fmt::{Display, Formatter};

/// Regular grid structure.
pub struct Grid {
    /// Boundary.
    bound: Aabb,
    /// Resolution.
    res: [usize; 3],
    /// Voxel size.
    voxel_size: Vec3,
}

impl Grid {
    access!(bound, Aabb);
    access!(res, [usize; 3]);
    access!(voxel_size, Vec3);

    /// Construct a new instance.
    /// # Errors
    /// if the cell array can not be constructed.
    #[inline]
    pub fn new(sett: &Settings) -> Result<Self, Error> {
        let mut voxel_size = sett.bound().widths();
        for (w, n) in voxel_size.iter_mut().zip(sett.res()) {
            *w /= *n as f64;
        }

        Ok(Self {
            bound: sett.bound().clone(),
            res: *sett.res(),
            voxel_size,
        })
    }

    /// Determine the total number of cells.
    #[inline]
    #[must_use]
    pub const fn total_cells(&self) -> usize {
        self.res[X] * self.res[Y] * self.res[Z]
    }

    /// If the given position is contained within the grid,
    /// generate the index for the given position within the grid.
    #[inline]
    #[must_use]
    pub fn gen_index(&self, p: &Pos3) -> Option<[usize; 3]> {
        if self.bound.contains(p) {
            let mins = self.bound.mins();
            let maxs = self.bound.maxs();
            Some([
                (((p.x - mins.x) / (maxs.x - mins.x)) * self.res[X] as f64).floor() as usize,
                (((p.y - mins.y) / (maxs.y - mins.y)) * self.res[Y] as f64).floor() as usize,
                (((p.z - mins.z) / (maxs.z - mins.z)) * self.res[Z] as f64).floor() as usize,
            ])
        } else {
            None
        }
    }

    /// If the given position is contained within the grid,
    /// generate the index and voxel for the given position within the grid.
    #[inline]
    #[must_use]
    pub fn gen_index_voxel(&self, p: &Pos3) -> Option<([usize; 3], Aabb)> {
        if let Some(index) = self.gen_index(p) {
            let mut min = *self.bound.mins();
            min.x += self.voxel_size[X] * index[X] as f64;
            min.y += self.voxel_size[Y] * index[Y] as f64;
            min.x += self.voxel_size[Z] * index[Z] as f64;

            Some((index, Aabb::new(min, min + self.voxel_size)))
        } else {
            None
        }
    }
}

impl Display for Grid {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        writeln!(
            fmt,
            "{}",
            report::obj("boundary", &self.bound).expect("Could not format field.")
        )?;

        writeln!(
            fmt,
            "{}",
            report::obj(
                "resolution",
                format!("{}x{}x{}", self.res[X], self.res[Y], self.res[Z])
            )
            .expect("Could not format field.")
        )?;

        write!(
            fmt,
            "{}",
            report::obj("total cells", self.total_cells()).expect("Could not format field.")
        )
    }
}
