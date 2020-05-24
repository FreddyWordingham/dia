//! Test photon lifetime function.

use crate::{
    mcrt::{Data, Hit, Input, Photon, Properties},
    Trace,
};
use rand::{rngs::ThreadRng, Rng};

/// Test photon lifetime function.
#[allow(clippy::option_expect_used)]
#[inline]
pub fn test(input: &Input, data: &mut Data, rng: &mut ThreadRng) {
    let (phot, mat) = emit_phot(input, rng);
    let env = mat.env(phot.wavelength());

    if let Some((index, voxel)) = input.grid.gen_index_voxel(phot.ray().pos()) {
        data.emitted_photons[index] += phot.weight();
        let voxel_dist = voxel
            .dist(phot.ray())
            .expect("Could not determine voxel distance.");
        let scat_dist = -(rng.gen_range(0.0_f64, 1.0)).ln() / env.inter_coeff();

        match Hit::new(voxel_dist, scat_dist) {
            Hit::Voxel(dist) => {}
            Hit::Scattering(dist) => {}
        }
    }
}

/// Generate a new photon.
#[inline]
#[must_use]
fn emit_phot<'a>(input: &'a Input, rng: &mut ThreadRng) -> (Photon, &'a Properties) {
    let phot = input.light.emit(input.sett.num_phot(), rng);
    let prop = &input.props.map()["air"];

    (phot, prop)
}
