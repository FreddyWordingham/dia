//! Regular grid settings implementation.

use crate::{access, report, Aabb, X, Y, Z};
use attr::load;
use std::fmt::{Display, Formatter};

/// Loadable triangle mesh conglomerate structure.
#[load]
pub struct Settings {
    /// Boundary.
    bound: Aabb,
    /// Resolution.
    res: [usize; 3],
}

impl Settings {
    access!(bound, Aabb);
    access!(res, [usize; 3]);

    /// Determine the total number of cells.
    #[inline]
    #[must_use]
    pub const fn total_cells(&self) -> usize {
        self.res[X] * self.res[Y] * self.res[Z]
    }
}

impl Display for Settings {
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
            report::obj("total cells", self.total_cells()).expect("Could not format field.")
        )?;

        write!(
            fmt,
            "{}",
            report::obj(
                "resolution",
                format!("{}x{}x{}", self.res[X], self.res[Y], self.res[Z])
            )
            .expect("Could not format field.")
        )
    }
}
