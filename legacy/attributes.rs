//! Rendering attributes form implementation.

use crate::{render::Attribute, Group, Set};
use attr::load;

/// Loadable attributes structure.
#[load]
pub struct Attributes {
    /// Attribute mappings.
    attributes: Vec<(Group, Attribute)>,
}

impl Attributes {
    /// Build an attribute set.
    #[inline]
    #[must_use]
    pub fn build(&self) -> Set<Attribute> {
        let mut attributes = Set::new();

        for (group, attr) in &self.attributes {
            if attributes.contains_key(group) {
                panic!("Duplicate attribute for group: {}", group);
            }

            attributes.insert(*group, attr.clone());
        }

        attributes
    }
}
