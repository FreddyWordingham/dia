//! Camera form implementation.

use crate::{
    render::{Colours as ColoursInst},
};
use attr::load;
use palette::{Gradient, LinSrgba};
use std::collections::BTreeMap;

/// Loadable colours structure.
#[load]
pub struct Colours {
    /// Gradient mappings.
    grads: Vec<(Group, Vec<[f32; 4]>)
}

impl Colours {
    /// Build a colour set.
    #[inline]
    #[must_use]
    pub fn build(&self) -> ColoursInst {
        let mut grads = BTreeMap::new();

        for (group, cols) in &self.grads {
            if grads.contains_key(group) {
                panic!("Duplicate gradient for group: {}", group);
            }

            grads.insert(
                *group,
                Gradient::new(
                    cols.iter()
                        .map(|col| {
                            LinSrgba::new(
                                col[0] as f32,
                                col[1] as f32,
                                col[2] as f32,
                                col[3] as f32,
                            )
                        })
                        .collect::<Vec<_>>(),
                ),
            );
        }

        ColoursInst::new(grads)
    }
}
