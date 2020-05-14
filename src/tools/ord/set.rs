//! Set implementation.

use crate::{access, as_json, from_json, Build, Error, Group, Load, Save};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fmt::{Display, Formatter},
    path::Path,
};

/// Set alias.
type Map<T> = BTreeMap<Group, T>;

/// Set map.
// #[load]
#[derive(Debug, Deserialize, Serialize)]
pub struct Set<T> {
    /// Internal mapping.
    map: Map<T>,
}

impl<T> Set<T> {
    access!(map, Map<T>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(map: Map<T>) -> Self {
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

impl<T> Load for Set<T>
where
    for<'de> T: Deserialize<'de>,
{
    #[inline]
    fn load(path: &Path) -> Result<Self, Error> {
        from_json(path)
    }
}

impl<T: Serialize> Save for Set<T> {
    #[inline]
    fn save(&self, path: &Path) -> Result<(), Error> {
        as_json(self, path)
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

impl<T: Display> Display for Set<T> {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        let mut items = self.map.iter();

        if let Some((group, item)) = items.next() {
            write!(fmt, "{:>16} >  {}", format!("[{}]", group), item)?;
        }

        for (group, item) in items {
            write!(fmt, "\n{:>16} >  {}", format!("[{}]", group), item)?;
        }

        Ok(())
    }
}
