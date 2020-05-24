//! Test photon lifetime function.

use crate::{
    distribution,
    mcrt::{Data, Event, Input, Optics, Photon, Properties},
    Trace,
};
use rand::{rngs::ThreadRng, Rng};
use std::f64::consts::PI;

/// Test photon lifetime function.
#[allow(clippy::option_expect_used)]
#[inline]
pub fn test(input: &Input, data: &mut Data, rng: &mut ThreadRng) {
    // Useful constants.
    let bump_dist = input.sett.bump_dist();

    // Photon variable initialisation.
    let (mut phot, mat) = emit_phot(input, rng);
    let env = mat.env(phot.wavelength());

    // Check photon can be placed within the grid domain.
    if let Some(index) = input.grid.gen_index(phot.ray().pos()) {
        data.emitted_photons[index] += phot.weight();
    } else {
        panic!("Photon was not emitted within the grid.");
    }

    // Loop photon life until it leaves the grid.
    while let Some((index, voxel)) = input.grid.gen_index_voxel(phot.ray().pos()) {
        //

        // println!("mins  : {}", voxel.mins());
        // println!("maxs  : {}", voxel.maxs());
        // println!("pos   : {}\n", phot.ray().pos());

        // Determine possible event distances.
        let voxel_dist = voxel
            .dist(phot.ray())
            .expect("Could not determine voxel distance.");
        let scat_dist = -(rng.gen_range(0.0_f64, 1.0)).ln() / env.inter_coeff();
        let surf_dist = input.tree.observe(
            phot.ray().clone(),
            input.sett.bump_dist(),
            voxel_dist.min(scat_dist),
        );

        // Handle event.
        match Event::new(voxel_dist, scat_dist, bump_dist) {
            Event::Voxel(dist) => move_phot(data, index, &mut phot, dist + bump_dist),
            Event::Scattering(dist) => {
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
    let prop = &input.props.map()["fog"];

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

    let phi = distribution::henyey_greenstein(rng, env.asym());
    data.rotations[index] += phi;

    phot.ray_mut().rotate(phi, rng.gen_range(0.0, 2.0 * PI));
}
