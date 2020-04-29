//! Camera form implementation.

use crate::{Group, Set};
use attr::load;
use palette::{Gradient, LinSrgba};

/// Loadable colours structure.
#[load]
pub struct Colours {
    /// Gradient mappings.
    grads: Vec<(Group, Vec<String>)>,
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
                            let col = hex::decode(col.replace("#", "")).unwrap();

                            LinSrgba::new(
                                f32::from(col[0]),
                                f32::from(col[1]),
                                f32::from(col[2]),
                                f32::from(col[3]),
                            )
                        })
                        .collect::<Vec<_>>(),
                ),
            );
        }

        grads
    }
}
