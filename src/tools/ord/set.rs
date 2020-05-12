//! Set implementation.

use crate::Group;
use std::collections::BTreeMap;

/// Set alias.
pub type Set<T> = BTreeMap<Group, T>;

/// Build a set from a list.
#[inline]
#[must_use]
pub fn set_from_list<T>(list: Vec<(Group, T)>) -> Set<T> {
    let mut set = Set::new();

    for (group, item) in list {
        if set.contains_key(&group) {
            panic!("Duplicate entries for group: {}", group);
        }

        set.insert(group, item);
    }

    set
}
