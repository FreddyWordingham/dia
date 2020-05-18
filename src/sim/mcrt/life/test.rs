//! Test photon lifetime function.

use crate::mcrt::{Data, Input};
use rand::rngs::ThreadRng;

/// Test photon lifetime function.
#[inline]
pub fn test(_input: &Input, data: &mut Data, _rng: &mut ThreadRng) {
    data.emitted_photons += 1.0;

    
}
