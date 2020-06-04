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
    _data: &mut Output,
    _weight: f64,
    _pixel: (usize, usize),
    _ray: Ray,
) {
}
