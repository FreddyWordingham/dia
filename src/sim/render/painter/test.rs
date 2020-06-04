//! Test painter function.

use crate::{
    render::{Input, Output},
    Ray,
};
use rand::rngs::ThreadRng;

/// Test painter function.
#[allow(clippy::option_expect_used)]
#[inline]
pub fn test(
    _thread_id: usize,
    _rng: &mut ThreadRng,
    _input: &Input,
    data: &mut Output,
    _weight: f64,
    pixel: [usize; 2],
    _ray: Ray,
) {
    data.image[pixel] += palette::LinSrgba::default();
}
