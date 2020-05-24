//! Test photon lifetime function.

use crate::mcrt::{Data, Input};
use rand::rngs::ThreadRng;

/// Test photon lifetime function.
#[inline]
pub fn test(input: &Input, data: &mut Data, rng: &mut ThreadRng) {
    data.emitted_photons += 1.0;

    let phot = input.light.emit(input.sett.num_phot(), rng);
    // let mat_key = &"air".to_string();
    let _mat = &input.props.map()["air"];

    if let Some((_index, _voxel)) = input.grid.gen_index_voxel(phot.ray().pos()) {
        
    }
}
