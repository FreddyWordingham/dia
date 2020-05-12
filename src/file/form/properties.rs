//! Physical properties form implementation.

use crate::{mcrt::Properties, Group, Set};
use attr::load;

/// Loadable attributes structure.
#[load]
pub struct Properties {
    /// Properties mappings.
    props: Vec<(Group, Properties)>,
}

impl Properties {
    /// Build an attribute set.
    #[inline]
    #[must_use]
    pub fn build(&self) -> Set<Properties> {
        let mut props = Set::new();

        for (group, attr) in &self.props {
            if props.contains_key(group) {
                panic!("Duplicate properties for group: {}", group);
            }

            props.insert(*group, attr.clone());
        }

        props
    }
}
