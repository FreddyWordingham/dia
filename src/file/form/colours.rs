//! Camera form implementation.

use crate::{Group, Set};
use attr::load;
use palette::{Gradient, LinSrgba};

/// Loadable colours structure.
#[load]
pub struct Colours {
    /// Gradient mappings.
    grads: Vec<(Group, Vec<[f32; 4]>)>,
}

impl Colours {
    /// Build a colour set.
    #[inline]
    #[must_use]
    pub fn build(&self) -> Set<Gradient<LinSrgba>> {
        let mut grads = Set::new();

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

        grads
    }
}
