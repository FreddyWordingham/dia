//! Set implementation.

use crate::Group;
use std::collections::BTreeMap;

/// Set alias.
pub type Set<T> = BTreeMap<Group, T>;

// /// Build an attribute set.
// #[inline]
// #[must_use]
// pub fn build(&self) -> Set<Properties> {
//     let mut props = Set::new();

//     for (group, attr) in &self.props {
//         if props.contains_key(group) {
//             panic!("Duplicate properties for group: {}", group);
//         }

//         props.insert(*group, attr.clone());
//     }

//     props
// }
