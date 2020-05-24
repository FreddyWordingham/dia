//! Hit-scan methods.

use crate::{tree::Cell, Hit, Ray};

impl<'a> Cell<'a> {
    /// Determine what a ray would observe within the cell.
    #[inline]
    #[must_use]
    pub fn observe(&self, ray: &Ray) -> Option<Hit> {
        None
    }
}
