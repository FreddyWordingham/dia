//! Render engine function.

use crate::{
    render::{
        engine::naboo::{light, shadow, visibility},
        Attributes, Event, Input, Scene,
    },
    Crossing, Dir3, Hit, Pos3, Ray, Trace,
};
use palette::LinSrgba;
use rand::rngs::ThreadRng;

/// Naboo rendering engine function.
#[allow(clippy::never_loop)]
#[allow(clippy::option_expect_used)]
#[allow(clippy::single_match_else)]
#[allow(clippy::too_many_lines)]
#[inline]
#[must_use]
pub fn engine(
    mut rng: &mut ThreadRng,
    input: &Input,
    scene: &Scene,
    mut ray: Ray,
    mut weight: f64,
) -> LinSrgba {
    debug_assert!(weight > 0.0);
    debug_assert!(weight <= 1.0);

    let bump_dist = input.sett.bump_dist();
    let fog_dist = scene.fog().dist();
    let mut col = LinSrgba::default();
    let mut fog = 0.0;

    // Move rays into the grid.
    if !input.grid.boundary().contains(ray.pos()) {
        if let Some(dist) = input.grid.boundary().dist(&ray) {
            ray.travel(dist + bump_dist)
        }
    }

    if let Some((_index, voxel)) = input.grid.gen_index_voxel(ray.pos()) {
        loop {
            // Determine possible event distances.
            let voxel_dist = voxel
                .dist(&ray)
                .expect("Could not determine voxel distance.");
            let surf_hit = input.tree.observe(ray.clone(), bump_dist, voxel_dist);

            // Handle event.
            match Event::new(voxel_dist, surf_hit) {
                Event::Voxel(dist) => {
                    if dist > fog_dist {
                        ray.travel(fog_dist / 2.0);
                        let sun_dir = Dir3::new_normalize(scene.light().sun_pos() - ray.pos());
                        let light_ray = Ray::new(*ray.pos(), sun_dir);
                        fog += visibility(input, light_ray, bump_dist, 1.0);
                        ray.travel(fog_dist / 2.0);
                        continue;
                    }

                    ray.travel(dist);
                    col += sky_col(scene, &ray, &input.cols.map()["sky"]) * weight as f32;
                    break;
                }
                Event::Surface(hit) => {
                    if hit.dist() > fog_dist {
                        ray.travel(fog_dist / 2.0);
                        let sun_dir = Dir3::new_normalize(scene.light().sun_pos() - ray.pos());
                        let light_ray = Ray::new(*ray.pos(), sun_dir);
                        fog += visibility(input, light_ray, bump_dist, 1.0);
                        ray.travel(fog_dist / 2.0);
                        continue;
                    }

                    let group = hit.group();
                    if let Some(attr) = input.attrs.map().get(group) {
                        match attr {
                            Attributes::Luminous { mult } => {
                                ray.travel(hit.dist());
                                let sun_dir =
                                    Dir3::new_normalize(Pos3::origin() - scene.light().sun_pos());
                                col += colour(&mut rng, input, scene, &ray, &hit, &sun_dir)
                                    * (mult * weight) as f32;
                                break;
                            }
                            Attributes::Transparent { abs } => {
                                ray.travel(hit.dist());
                                let sun_dir =
                                    Dir3::new_normalize(ray.pos() - scene.light().sun_pos());
                                col += colour(&mut rng, input, scene, &ray, &hit, &sun_dir)
                                    * *abs as f32
                                    * weight as f32;
                                weight *= 1.0 - *abs;
                                ray.travel(bump_dist);
                            }
                            Attributes::Mirror { abs } => {
                                ray.travel(hit.dist());
                                let sun_dir =
                                    Dir3::new_normalize(ray.pos() - scene.light().sun_pos());
                                col += colour(&mut rng, input, scene, &ray, &hit, &sun_dir)
                                    * (*abs * weight) as f32;
                                weight *= 1.0 - *abs;
                                *ray.dir_mut() = Crossing::init_ref_dir(
                                    ray.dir(),
                                    hit.side().norm(),
                                    -ray.dir().dot(hit.side().norm()),
                                );
                                ray.travel(bump_dist);
                            }
                            Attributes::Solar {
                                abs,
                                inside,
                                outside,
                            } => {
                                ray.travel(hit.dist());
                                let sun_dir = crate::Vec3::z_axis();
                                col += colour(&mut rng, input, scene, &ray, &hit, &sun_dir)
                                    * (*abs * weight) as f32;
                                weight *= 1.0 - abs;

                                let (n_curr, n_next) = if hit.side().is_inside() {
                                    (*inside, *outside)
                                } else {
                                    (*outside, *inside)
                                };
                                let crossing =
                                    Crossing::new(ray.dir(), hit.side().norm(), n_curr, n_next);

                                let trans_prob = crossing.trans_prob();
                                if let Some(trans_dir) = crossing.trans_dir() {
                                    let mut trans_ray = ray.clone();
                                    *trans_ray.dir_mut() = *trans_dir;
                                    trans_ray.travel(bump_dist);
                                    col +=
                                        engine(rng, input, scene, trans_ray, weight * trans_prob)
                                            * weight as f32;
                                }

                                weight *= crossing.ref_prob();
                                *ray.dir_mut() = *crossing.ref_dir();
                                ray.travel(bump_dist);
                            }
                            Attributes::Refractive {
                                abs,
                                inside,
                                outside,
                            } => {
                                ray.travel(hit.dist());
                                let sun_dir =
                                    Dir3::new_normalize(ray.pos() - scene.light().sun_pos());
                                col += colour(&mut rng, input, scene, &ray, &hit, &sun_dir)
                                    * (*abs * weight) as f32;
                                weight *= 1.0 - abs;

                                let (n_curr, n_next) = if hit.side().is_inside() {
                                    (*inside, *outside)
                                } else {
                                    (*outside, *inside)
                                };
                                let crossing =
                                    Crossing::new(ray.dir(), hit.side().norm(), n_curr, n_next);

                                let trans_prob = crossing.trans_prob();
                                if let Some(trans_dir) = crossing.trans_dir() {
                                    let mut trans_ray = ray.clone();
                                    *trans_ray.dir_mut() = *trans_dir;
                                    trans_ray.travel(bump_dist);
                                    col +=
                                        engine(rng, input, scene, trans_ray, weight * trans_prob)
                                            * weight as f32;
                                }

                                weight *= crossing.ref_prob();
                                *ray.dir_mut() = *crossing.ref_dir();
                                ray.travel(bump_dist);
                            }
                        }
                    } else {
                        ray.travel(hit.dist());
                        let sun_dir = Dir3::new_normalize(ray.pos() - scene.light().sun_pos());
                        col += colour(&mut rng, input, scene, &ray, &hit, &sun_dir) * weight as f32;
                        break;
                    }
                }
            }

            if weight < input.sett.min_weight() {
                break;
            }
        }
    } else {
        col += sky_col(scene, &ray, &input.cols.map()["sky"]);
    }

    col += input.cols.map()["fog"].get(1.0)
        * (scene.fog().scale() * (fog * fog_dist)).powi(scene.fog().power()) as f32;

    col
}

/// Perform a colouring.
#[inline]
fn colour(
    rng: &mut ThreadRng,
    input: &Input,
    scene: &Scene,
    ray: &Ray,
    hit: &Hit,
    sun_dir: &Dir3,
) -> LinSrgba {
    let light = (light(scene, ray, hit) + 0.5).min(1.0);
    let shadow = shadow(input, scene, ray, hit, input.sett.bump_dist(), rng);

    let base_col = input.cols.map()[hit.group()].get(hit.side().norm().dot(&sun_dir).abs() as f32);
    let grad = palette::Gradient::new(vec![palette::LinSrgba::default(), base_col]);

    grad.get((light * shadow) as f32)
}

/// Determine the sky colour.
#[inline]
#[must_use]
fn sky_col(
    scene: &Scene,
    ray: &Ray,
    grad: &palette::Gradient<palette::LinSrgba>,
) -> palette::LinSrgba {
    let u = (ray.dir().dot(scene.cam().focus().orient().up()) + 1.0) * 0.5;
    let v = (ray.dir().dot(scene.cam().focus().orient().right()) + 1.0) * 0.5;

    let x = (scene.perlin().sample(u, v) + 1.0) * 0.5;

    let col = grad.get(x as f32);

    palette::Gradient::new(vec![palette::LinSrgba::default(), col])
        .get(scene.light().sky_brightness())
}
