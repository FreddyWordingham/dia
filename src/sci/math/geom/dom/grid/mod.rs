//! Regular grid cell scheme.

pub mod settings;
pub mod voxel;

pub use self::{settings::*, voxel::*};

use crate::{access, report, Aabb, Error, Vec3, X, Y, Z};
use ndarray::Array3;
use std::fmt::{Display, Formatter};

/// Regular grid structure.
pub struct Grid {
    /// Boundary.
    bound: Aabb,
    /// Cells.
    cells: Array3<Voxel>,
}

impl Grid {
    access!(bound, Aabb);
    access!(cells, Array3<Voxel>);

    /// Construct a new instance.
    /// # Errors
    /// if the cell array can not be constructed.
    #[inline]
    pub fn new(sett: &Settings) -> Result<Self, Error> {
        let mut cell_size = sett.bound().widths();
        for (w, n) in cell_size.iter_mut().zip(sett.res()) {
            *w /= *n as f64;
        }

        let bound_mins = sett.bound().mins();

        let res = sett.res();
        let mut cells = Vec::with_capacity(res[X] * res[Y] * res[Z]);
        for xi in 0..res[X] {
            let x = cell_size.x * xi as f64;

            for yi in 0..res[Y] {
                let y = cell_size.y * yi as f64;

                for zi in 0..res[Z] {
                    let z = cell_size.z * zi as f64;

                    let mins = bound_mins + Vec3::new(x, y, z);
                    let maxs = mins + cell_size;

                    cells.push(Voxel::new(Aabb::new(mins, maxs)));
                }
            }
        }

        Ok(Self {
            bound: sett.bound().clone(),
            cells: Array3::from_shape_vec([res[X], res[Y], res[Z]], cells)?,
        })
    }

    /// Get the grid resolution.
    #[inline]
    #[must_use]
    pub fn res(&self) -> [usize; 3] {
        let shape = self.cells.shape();
        [shape[X], shape[Y], shape[Z]]
    }

    /// Determine the total number of cells.
    #[inline]
    #[must_use]
    pub fn total_cells(&self) -> usize {
        let res = self.res();
        res[X] * res[Y] * res[Z]
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
                format!(
                    "{}x{}x{}",
                    self.cells.shape()[X],
                    self.cells.shape()[Y],
                    self.cells.shape()[Z]
                )
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
