//! Regular grid cell scheme.

pub mod settings;

pub use self::settings::*;

use crate::{access, report, Aabb, Error, Vec3, X, Y, Z};
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
    pub fn total_cells(&self) -> usize {
        self.res[X] * self.res[Y] * self.res[Z]
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
