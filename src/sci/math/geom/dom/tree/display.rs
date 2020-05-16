//! Display trait implementation

use crate::{
    report,
    tree::{Cell, Settings},
};
use std::fmt::{Display, Formatter, Result, Write};

impl<'a> Display for Cell<'a> {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        let name = match self {
            Self::Root { .. } => "Root",
            Self::Branch { .. } => "Branch",
            Self::Leaf { .. } => "Leaf",
            Self::Empty { .. } => "Empty",
        };
        writeln!(
            fmt,
            "{}",
            report::obj("type", name).expect("Could not format name.")
        )?;

        write!(
            fmt,
            "{}",
            report::obj(
                "depth", // self.depth()
                -9
            )
            .expect("Could not format field.")
        )
    }
}
