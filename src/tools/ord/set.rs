//! Set implementation.

use crate::{access, Build, Error, Group};
use std::{collections::BTreeMap, path::Path};

/// Set alias.
type Map<T> = BTreeMap<Group, T>;

/// Set map.
pub struct Set<T> {
    /// Internal mapping.
    map: BTreeMap<Group, T>,
}

impl<T> Set<T> {
    access!(map, BTreeMap<Group, T>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(map: BTreeMap<Group, T>) -> Self {
        debug_assert!(!map.is_empty());

        Self { map }
    }

    /// Construct an instance from a vector.
    #[inline]
    #[must_use]
    pub fn from_vec(list: Vec<(Group, T)>) -> Self {
        let mut map = Map::new();

        for (group, item) in list {
            if map.contains_key(&group) {
                panic!("Duplicate entries for group: {}", group);
            }

            map.insert(group, item);
        }

        Self::new(map)
    }
}

impl<T: Build> Build for Set<T> {
    type Inst = Set<T::Inst>;

    #[inline]
    fn build(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        let mut map = Map::new();

        for (group, item) in self.map {
            if map.contains_key(&group) {
                panic!("Duplicate entries for group: {}", group);
            }

            map.insert(group, item.build(in_dir)?);
        }

        Ok(Self::Inst::new(map))
    }
}
