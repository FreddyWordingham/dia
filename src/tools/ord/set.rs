//! Set implementation.

use crate::Group;
use std::collections::BTreeMap;

/// Set alias.
pub type Set<T> = BTreeMap<Group, T>;
