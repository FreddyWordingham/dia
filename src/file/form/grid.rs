//! Grid form implementation.

use crate::{report, Aabb, X, Y, Z};
use attr::load;
use std::fmt::{Display, Formatter};

/// Loadable triangle mesh conglomerate structure.
#[load]
pub struct Grid {
    /// Boundary.
    bound: Aabb,
    /// Resolution.
    res: [usize; 3],
}

impl Grid {
    /// Determine the total number of cells.
    #[inline]
    #[must_use]
    pub const fn total_cells(&self) -> usize {
        self.res[X] * self.res[Y] * self.res[Z]
    }
}

// impl Build for Grid {
//     type Inst = crate::Grid;

//     #[inline]
//     fn build(self, in_dir: &Path) -> Result<Self::Inst, Error> {
//         let mut tris = Vec::new();
//         for name in self.0 {
//             let obj = Self::Inst::load(&in_dir.join(name))?;
//             tris.extend(obj.into_tris())
//         }

//         Ok(Self::Inst::new(tris))
//     }
// }

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
