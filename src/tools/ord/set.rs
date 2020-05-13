//! Set implementation.

use crate::{Build, Error, Group};
use std::collections::BTreeMap;
use std::path::Path;

/// Set alias.
pub type Set<T> = BTreeMap<Group, T>;

/// Convert a set from a list.
#[inline]
#[must_use]
pub fn convert_from_list<T>(list: Vec<(Group, T)>) -> Set<T> {
    let mut set = Set::new();

    for (group, item) in list {
        if set.contains_key(&group) {
            panic!("Duplicate entries for group: {}", group);
        }

        set.insert(group, item);
    }

    set
}

/// Build a set from a list.
/// # Errors
/// if an item could not be built.
#[inline]
pub fn build_from_list<T: Build>(
    in_dir: &Path,
    list: Vec<(Group, T)>,
) -> Result<Set<T::Inst>, Error> {
    let mut set = Set::new();

    for (group, item) in list {
        if set.contains_key(&group) {
            panic!("Duplicate entries for group: {}", group);
        }

        set.insert(group, item.build(in_dir)?);
    }

    Ok(set)
}
