//! Test photon lifetime function.

use crate::mcrt::{Data, Input};
use rand::rngs::ThreadRng;

/// Test photon lifetime function.
#[inline]
pub fn test(input: &Input, data: &mut Data, rng: &mut ThreadRng) {
    data.emitted_photons += 1.0;

    let phot = input.light.emit(rng);
}
