//! Test photon lifetime function.

use crate::{
    mcrt::{Event, Input, Output, Photon, Properties},
    Trace,
};
use rand::{rngs::ThreadRng, Rng};

/// Test photon lifetime function.
#[allow(clippy::option_expect_used)]
#[inline]
pub fn test(input: &Input, data: &mut Output, rng: &mut ThreadRng) {
    // Useful constants.
    let bump_dist = input.sett.bump_dist();

    // Photon variable initialisation.
    let (phot, mat) = emit_phot(input, rng);
    let env = mat.env(phot.wavelength());

    // Check photon can be placed within the grid domain.
    if let Some(index) = input.grid.gen_index(phot.ray().pos()) {
        data.emitted_photons[index] += phot.weight();
    } else {
        panic!("Photon was not emitted within the grid.");
    }

    // Loop photon life until it leaves the grid.
    while let Some((_index, voxel)) = input.grid.gen_index_voxel(phot.ray().pos()) {
        // Determine possible event distances.
        let voxel_dist = voxel
            .dist(phot.ray())
            .expect("Could not determine voxel distance.");
        let scat_dist = -(rng.gen_range(0.0_f64, 1.0)).ln() / env.inter_coeff();
        let surf_hit = input.tree.observe(
            phot.ray().clone(),
            input.sett.bump_dist(),
            voxel_dist.min(scat_dist),
        );

        // Handle event.
        match Event::new(voxel_dist, scat_dist, surf_hit, bump_dist) {
            Event::Voxel(_dist) => break,
            Event::Scattering(_dist) => break,
            Event::Surface(_hit) => break,
        }
    }
}

/// Generate a new photon.
#[inline]
#[must_use]
fn emit_phot<'a>(input: &'a Input, rng: &mut ThreadRng) -> (Photon, &'a Properties) {
    let phot = input.light.emit(input.sett.num_phot(), rng);
    // while phot.wavelength() <= input.sett.wavelength_limits.min() || phot.wavelength() >= 800.0e-9 {
    //     phot = input.light.emit(input.sett.num_phot(), rng);
    // }

    let prop = &input.props.map()[input.sett.init_mat()];

    (phot, prop)
}
