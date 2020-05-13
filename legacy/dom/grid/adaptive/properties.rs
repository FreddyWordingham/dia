//! Information methods.

impl<'a> super::Adaptive<'a> {
    /// Determine the total number of cells.
    #[inline]
    #[must_use]
    pub fn max_depth(&self) -> usize {
        match self {
            Self::Root { children, .. } => children.iter().map(|c| c.max_depth()).max().unwrap(),
            Self::Branch { children, .. } => {
                1 + children.iter().map(|c| c.max_depth()).max().unwrap()
            }
            Self::Leaf { .. } | Self::Empty { .. } => 1,
        }
    }

    /// Determine the total number of cells.
    #[inline]
    #[must_use]
    pub fn num_cells(&self) -> usize {
        match self {
            Self::Root { children, .. } | Self::Branch { children, .. } => {
                1 + children.iter().map(|ch| ch.num_cells()).sum::<usize>()
            }
            Self::Leaf { .. } | Self::Empty { .. } => 1,
        }
    }

    /// Determine the total number of leaf cells.
    #[inline]
    #[must_use]
    pub fn num_leaf_cells(&self) -> usize {
        match self {
            Self::Root { children, .. } | Self::Branch { children, .. } => {
                children.iter().map(|ch| ch.num_leaf_cells()).sum::<usize>()
            }
            Self::Leaf { .. } => 1,
            Self::Empty { .. } => 0,
        }
    }

    /// Determine the average number of triangles in each leaf cell.
    #[inline]
    #[must_use]
    pub fn num_tri_refs(&self) -> usize {
        match self {
            Self::Root { children, .. } | Self::Branch { children, .. } => {
                children.iter().map(|c| c.num_tri_refs()).sum()
            }
            Self::Leaf { tris, .. } => tris.len(),
            Self::Empty { .. } => 0,
        }
    }

    /// Determine the average number of triangles in each leaf cell.
    #[inline]
    #[must_use]
    pub fn ave_leaf_tris(&self) -> f64 {
        self.num_tri_refs() as f64 / self.num_leaf_cells() as f64
    }
}
