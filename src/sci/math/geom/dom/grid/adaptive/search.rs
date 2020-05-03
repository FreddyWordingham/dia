//! Scan methods.

use crate::Pos3;

impl<'a> super::Adaptive<'a> {
    /// Determine the terminal cell containing the given position.
    #[inline]
    #[must_use]
    pub fn find_terminal_cell(&self, pos: &Pos3) -> Option<&Self> {
        debug_assert!(self.boundary().contains(pos));

        match self {
            Self::Leaf { .. } | Self::Empty { .. } => Some(self),
            Self::Root { boundary, children } | Self::Branch { boundary, children } => {
                let mut index = 0;
                let c = boundary.centre();

                if pos.x >= c.x {
                    index += 1;
                }
                if pos.y >= c.y {
                    index += 2;
                }
                if pos.z >= c.z {
                    index += 4;
                }
                children[index].find_terminal_cell(pos)
            }
        }
    }
}
