//! Test photon lifetime function.

use crate::{
    distribution,
    mcrt::{Environment, Event, Input, Output, Photon, Properties},
    Trace,
};
use rand::{rngs::ThreadRng, Rng};
use std::f64::consts::PI;

/// Test photon lifetime function.
#[allow(clippy::option_expect_used)]
#[inline]
pub fn test(input: &Input, data: &mut Output, rng: &mut ThreadRng) {
    // Useful constants.
    let bump_dist = input.sett.bump_dist();
    let loop_limit = input.sett.loop_limit();
    let roulette_weight = input.sett.roulette_weight();
    let roulette_survive_prob = 1.0 / input.sett.roulette_barrels() as f64;

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
    let mut loops = 0;
    while let Some((index, voxel)) = input.grid.gen_index_voxel(phot.ray().pos()) {
        // Check if loop limit has been reached.
        if loops >= loop_limit {
            println!("Warning! Terminating photon: loop limit reached.");
            break;
        }
        loops += 1;

        // Roulette.
        if phot.weight() <= roulette_weight {
            let r = rng.gen::<f64>();
            if r > roulette_survive_prob {
                break;
            }
            *phot.weight_mut() *= input.sett.roulette_barrels() as f64;
        }

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
            Event::Voxel(dist) => travel(data, index, &env, &mut phot, dist + bump_dist),
            Event::Scattering(dist) => {
                scatter(data, rng, index, &env, &mut phot, dist);
            }
            Event::Surface(hit) => travel(data, index, &env, &mut phot, hit.dist() + bump_dist),
        }
    }
}

/// Generate a new photon.
#[inline]
#[must_use]
fn emit_phot<'a>(input: &'a Input, rng: &mut ThreadRng) -> (Photon, &'a Properties) {
    let mut phot;
    loop {
        phot = input.light.emit(input.sett.num_phot(), rng);
        if input.sett.range().contains(phot.wavelength()) {
            break;
        }
    }

    let prop = &input.props.map()[input.sett.init_mat()];

    (phot, prop)
}

/// Move the photon forward and record the flight.
#[inline]
fn travel(data: &mut Output, index: [usize; 3], _env: &Environment, phot: &mut Photon, dist: f64) {
    debug_assert!(dist > 0.0);

    phot.ray_mut().travel(dist);
    data.dist_travelled[index] += dist;
}

/// Perform a photon scattering event.
#[inline]
fn scatter(
    data: &mut Output,
    rng: &mut ThreadRng,
    index: [usize; 3],
    env: &Environment,
    phot: &mut Photon,
    dist: f64,
) {
    debug_assert!(dist > 0.0);

    travel(data, index, env, phot, dist);

    *phot.weight_mut() *= env.albedo();
    let phi = distribution::henyey_greenstein(rng, env.asym());
    let theta = rng.gen_range(0.0, PI * 2.0);
    phot.ray_mut().rotate(phi, theta);
}
