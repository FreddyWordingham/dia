//! Test photon lifetime function.

use crate::{
    distribution,
    mcrt::{Environment, Event, Input, Material, Output, Photon},
    Crossing, Hit, Set, Trace,
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
    let mut env = mat.env(phot.wavelength());

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
            // Voxel boundary hit.
            Event::Voxel(dist) => travel(data, index, &env, &mut phot, dist + bump_dist),
            // Interaction event.
            Event::Scattering(dist) => {
                scatter(data, rng, index, &env, &mut phot, dist);
            }
            // Interface collision.
            Event::Surface(hit) => {
                // Move to the collision point.
                travel(data, index, &env, &mut phot, hit.dist());

                // Special collision events.
                // match hit.group() {
                //     "spectrometer" => {}
                //     _ => {}
                // }

                // Get the near, and far side refractive indices.
                let curr_ref = env.ref_index();
                let next_env = select_property(&hit, input.mats).env(phot.wavelength());
                let next_ref = next_env.ref_index();

                // Calculate the crossing normal vectors.
                let crossing =
                    Crossing::new(phot.ray().dir(), hit.side().norm(), curr_ref, next_ref);

                // Determine if a reflection or transmission occurs.
                let r = rng.gen::<f64>();
                if r <= crossing.ref_prob() {
                    // Reflect.
                    *phot.ray_mut().dir_mut() = *crossing.ref_dir();
                } else {
                    // Refract.
                    *phot.ray_mut().dir_mut() = crossing.trans_dir().expect("Invalid refraction.");
                    env = next_env;
                }

                // Move slightly away from the surface.
                travel(data, index, &env, &mut phot, input.sett.bump_dist());
            }
        }
    }
}

/// Generate a new photon.
#[inline]
#[must_use]
fn emit_phot<'a>(input: &'a Input, rng: &mut ThreadRng) -> (Photon, &'a Material) {
    // Generate photons from the light source until they're in the optical range of interest.
    let mut phot;
    loop {
        phot = input.light.emit(input.sett.num_phot(), rng);
        if input.sett.range().contains(phot.wavelength()) {
            break;
        }
    }

    // Select the required material.
    let mat = &input.mats.map()[input.sett.init_mat()];

    (phot, mat)
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

    // Move to the interaction point.
    travel(data, index, env, phot, dist);

    // Part of the weight is absorbed.
    *phot.weight_mut() *= env.albedo();

    // The remaining weight may be shifted in a Raman/fluorescence event.
    let r = rng.gen::<f64>();
    if r <= env.shift_prob() {
        // Shift occurs.
        // Fluorescence event removes photons from optical range of interest.
        *phot.weight_mut() = 0.0;
        return;
    }

    // The remaining weight is scattered.
    let phi = distribution::henyey_greenstein(rng, env.asym());
    let theta = rng.gen_range(0.0, PI * 2.0);
    phot.ray_mut().rotate(phi, theta);
}

/// Determine the next material from the hit event information.
#[must_use]
#[inline]
pub fn select_property<'a>(hit: &Hit, mats: &'a Set<Material>) -> &'a Material {
    match hit.group() {
        "skin" => {
            if hit.side().is_inside() {
                &mats.map()["air"]
            } else {
                &mats.map()["flesh"]
            }
        }
        "tumour_body" => {
            if hit.side().is_inside() {
                &mats.map()["flesh"]
            } else {
                &mats.map()["tumour"]
            }
        }
        "tumour_cap" => {
            if hit.side().is_inside() {
                &mats.map()["air"]
            } else {
                &mats.map()["tumour"]
            }
        }
        _ => panic!(format!(
            "Do not know how to handle collision with group {}",
            hit.group()
        )),
    }
}
