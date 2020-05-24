//! Test photon lifetime function.

use crate::{
    distribution,
    mcrt::{Data, Hit, Input, Optics, Photon, Properties},
    Trace,
};
use rand::{rngs::ThreadRng, Rng};
use std::f64::consts::PI;

/// Test photon lifetime function.
#[allow(clippy::option_expect_used)]
#[inline]
pub fn test(input: &Input, data: &mut Data, rng: &mut ThreadRng) {
    let (mut phot, mat) = emit_phot(input, rng);
    let env = mat.env(phot.wavelength());
    let bump_dist = input.sett.bump_dist();

    if let Some((index, voxel)) = input.grid.gen_index_voxel(phot.ray().pos()) {
        data.emitted_photons[index] += phot.weight();
        let voxel_dist = voxel
            .dist(phot.ray())
            .expect("Could not determine voxel distance.");
        let scat_dist = -(rng.gen_range(0.0_f64, 1.0)).ln() / env.inter_coeff();

        match Hit::new(voxel_dist, scat_dist, bump_dist) {
            Hit::Voxel(dist) => move_phot(data, index, &mut phot, dist + bump_dist),
            Hit::Scattering(dist) => {
                move_phot(data, index, &mut phot, dist);
                scatter_phot(data, index, &mut phot, &env, rng);
            }
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

/// Move the photon and record relevant data.
#[inline]
fn move_phot(data: &mut Data, index: [usize; 3], phot: &mut Photon, dist: f64) {
    debug_assert!(dist > 0.0);

    data.dist_travelled[index] += dist;

    phot.ray_mut().travel(dist);
}

/// Scatter the photon and record relevant data.
#[inline]
fn scatter_phot(
    data: &mut Data,
    index: [usize; 3],
    phot: &mut Photon,
    env: &Optics,
    rng: &mut ThreadRng,
) {
    data.scatters[index] += phot.weight();

    phot.ray_mut().rotate(
        distribution::henyey_greenstein(rng, env.asym()),
        rng.gen_range(0.0, 2.0 * PI),
    );
}
