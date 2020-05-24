//! Hit enumeration.

use crate::report;
use std::fmt::{Display, Formatter, Result};

/// Hit determination enumeration.
pub enum Hit {}

impl Hit {}

impl Display for Hit {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        let event = format!("Scatter {} [m]", 1.23);
        write!(
            fmt,
            "{}",
            report::obj("event", event).expect("Could not format field.")
        )
    }
}
