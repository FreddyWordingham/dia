//! Test photon lifetime function.

use crate::{
    distribution,
    mcrt::{Data, Environment, Event, Input, Photon, Properties},
    Crossing, Hit, Set, Trace,
};
use physical_constants::SPEED_OF_LIGHT_IN_VACUUM;
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
    let mut env = mat.env(phot.wavelength());

    // Check photon can be placed within the grid domain.
    if let Some(index) = input.grid.gen_index(phot.ray().pos()) {
        data.emitted_photons[index] += phot.weight();
    } else {
        panic!("Photon was not emitted within the grid.");
    }

    // Loop photon life until it leaves the grid.
    while let Some((index, voxel)) = input.grid.gen_index_voxel(phot.ray().pos()) {
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
            Event::Voxel(dist) => move_phot(data, index, &env, &mut phot, dist + bump_dist),
            Event::Scattering(dist) => {
                move_phot(data, index, &env, &mut phot, dist);
                scatter_phot(data, index, &mut phot, &env, rng);
            }
            Event::Surface(hit) => {
                match hit.group() {
                    "spectrometer_0" => {
                        move_phot(data, index, &env, &mut phot, hit.dist() + bump_dist);
                        data.spec_0.collect_weight(phot.wavelength(), phot.weight());
                        continue;
                    }
                    "spectrometer_1" => {
                        move_phot(data, index, &env, &mut phot, hit.dist() + bump_dist);
                        data.spec_1.collect_weight(phot.wavelength(), phot.weight());
                        continue;
                    }
                    "spectrometer_2" => {
                        move_phot(data, index, &env, &mut phot, hit.dist() + bump_dist);
                        data.spec_2.collect_weight(phot.wavelength(), phot.weight());
                        continue;
                    }
                    "spectrometer_3" => {
                        move_phot(data, index, &env, &mut phot, hit.dist() + bump_dist);
                        data.spec_3.collect_weight(phot.wavelength(), phot.weight());
                        continue;
                    }
                    "spectrometer_4" => {
                        move_phot(data, index, &env, &mut phot, hit.dist() + bump_dist);
                        data.spec_4.collect_weight(phot.wavelength(), phot.weight());
                        continue;
                    }
                    _ => {}
                }

                let curr_ref = env.ref_index();
                let next_env = select_property(input.props, &hit).env(phot.wavelength());
                let next_ref = next_env.ref_index();

                let crossing =
                    Crossing::new(phot.ray().dir(), hit.side().norm(), curr_ref, next_ref);

                if rng.gen_range(0.0, 1.0) <= crossing.ref_prob() {
                    // Reflect.
                    move_phot(
                        data,
                        index,
                        &env,
                        &mut phot,
                        (hit.dist() - bump_dist).max(0.0),
                    );
                    *phot.ray_mut().dir_mut() = *crossing.ref_dir();
                    *phot.weight_mut() *= crossing.ref_prob();
                } else {
                    // Refract
                    move_phot(data, index, &env, &mut phot, hit.dist() + bump_dist);
                    *phot.ray_mut().dir_mut() = crossing.trans_dir().expect("Invalid refraction.");
                    env = next_env;
                    *phot.weight_mut() *= crossing.trans_prob();
                }

                data.hits[index] += phot.weight();
            }
        }
    }
}

/// Generate a new photon.
#[inline]
#[must_use]
fn emit_phot<'a>(input: &'a Input, rng: &mut ThreadRng) -> (Photon, &'a Properties) {
    let mut phot = input.light.emit(input.sett.num_phot(), rng);
    while phot.wavelength() <= 450.0e-9 || phot.wavelength() >= 800.0e-9 {
        phot = input.light.emit(input.sett.num_phot(), rng);
    }

    let prop = &input.props.map()["air"];

    (phot, prop)
}

/// Move the photon and record relevant data.
#[inline]
fn move_phot(data: &mut Data, index: [usize; 3], env: &Environment, phot: &mut Photon, dist: f64) {
    debug_assert!(dist > 0.0);

    data.dist_travelled[index] += dist;
    data.energy[index] +=
        phot.weight() * phot.power() * (env.ref_index() / SPEED_OF_LIGHT_IN_VACUUM) * dist;
    data.absorptions[index] += phot.weight() * phot.power() * dist * env.abs_coeff();
    data.shifts[index] += phot.weight() * phot.power() * dist * env.shift_coeff();

    phot.ray_mut().travel(dist);
}

/// Scatter the photon and record relevant data.
#[inline]
fn scatter_phot(
    data: &mut Data,
    index: [usize; 3],
    phot: &mut Photon,
    env: &Environment,
    rng: &mut ThreadRng,
) {
    data.scatters[index] += phot.weight();

    let phi = distribution::henyey_greenstein(rng, env.asym());
    data.rotations[index] += phi;

    phot.ray_mut().rotate(phi, rng.gen_range(0.0, 2.0 * PI));

    *phot.weight_mut() *= env.albedo();
}

/// Determine the next property from the hit event information.
#[must_use]
#[inline]
pub fn select_property<'a>(props: &'a Set<Properties>, hit: &Hit) -> &'a Properties {
    let group = hit.group();
    match group {
        "skin" => {
            if hit.side().is_inside() {
                &props.map()["air"]
            } else {
                &props.map()["flesh"]
            }
        }
        "tumour" => {
            if hit.side().is_inside() {
                &props.map()["flesh"]
            } else {
                &props.map()["tumour"]
            }
        }
        "tumour_cap" => {
            if hit.side().is_inside() {
                &props.map()["air"]
            } else {
                &props.map()["tumour"]
            }
        }
        _ => panic!(format!(
            "Do not know how to handle collision with group {}",
            group
        )),
    }
}
