//! Camera form implementation.

use crate::{Error, Group, Set};
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
    /// # Errors
    /// if a a hexadecimal string can't be parsed.
    #[inline]
    pub fn build(&self) -> Result<Set<Gradient<LinSrgba>>, Error> {
        let mut grads = Set::new();

        for (group, cols) in &self.grads {
            if grads.contains_key(group) {
                panic!("Duplicate gradient for group: {}", group);
            }

            let mut cs = Vec::with_capacity(cols.len());
            for col in cols.iter() {
                let col_arr = hex::decode(col.replace("#", ""))?;

                let r = f32::from(col_arr[0]) / 255.0;
                let g = f32::from(col_arr[1]) / 255.0;
                let b = f32::from(col_arr[2]) / 255.0;
                let a = f32::from(col_arr[3]) / 255.0;

                cs.push(LinSrgba::new(r, g, b, a));
            }

            grads.insert(*group, Gradient::new(cs));
        }

        Ok(grads)
    }
}
