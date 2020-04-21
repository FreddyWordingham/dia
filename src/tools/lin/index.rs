//! Index producing functions.

use crate::Cartesian::{Y, Z};

/// Create the next three-dimensional index from the given linear index.
#[inline]
#[must_use]
pub fn three_dim(n: usize, res: [usize; 3]) -> [usize; 3] {
    let zi = n % res.get(Z as usize).unwrap();
    let yi = (n / res.get(Z as usize).unwrap()) % res.get(Y as usize).unwrap();
    let xi = n / (res.get(Y as usize).unwrap() * res.get(Z as usize).unwrap());

    [xi, yi, zi]
}
