//! Hit-scan painting function.

use crate::{
    render::{illumination, Scene},
    Crossing, Dir3, Pos3, Ray, Vec3,
};
use palette::{Gradient, LinSrgba};
use rand::rngs::ThreadRng;

/// Minimum fragment weight to simulate.
const MIN_WEIGHT: f64 = 0.01;

/// Mirror colouring fraction.
const MIRROR_COLOURING: f32 = 0.15;

/// Refractive index of refracting surfaces.
pub const REF_INDEX: f64 = 1.3;

/// Colour if surface is hit.
#[inline]
#[must_use]
pub fn colour(
    thread_id: usize,
    scene: &Scene,
    mut ray: Ray,
    rng: &mut ThreadRng,
    weight: f64,
) -> LinSrgba {
    debug_assert!(weight > 0.0);

    let mut col = LinSrgba::default();

    if weight <= MIN_WEIGHT {
        return col;
    }

    let mut sky = true;
    while let Some(hit) = scene.grid().observe(ray.clone(), scene.sett().bump_dist()) {
        ray.travel(hit.dist());

        let light = illumination::light(&ray, scene, &hit);
        let shadow = illumination::shadow(&ray, scene, &hit, rng);
        let illumination = light * shadow;

        let x = match hit.group() {
            3 => hit.side().norm().dot(&Vec3::z_axis()).abs(),
            _ => 1.0,
        };

        match hit.group() {
            1 => {
                let grad = Gradient::new(vec![
                    LinSrgba::default(),
                    scene.cols()[&hit.group()].get(x as f32),
                ]);
                col += grad.get(illumination as f32) * MIRROR_COLOURING;
                *ray.dir_mut() = Crossing::init_ref_dir(
                    ray.dir(),
                    hit.side().norm(),
                    -ray.dir().dot(hit.side().norm()),
                );
                ray.travel(scene.sett().bump_dist());
            }
            2 => {
                let grad = Gradient::new(vec![
                    LinSrgba::default(),
                    scene.cols()[&hit.group()].get(x as f32),
                ]);
                col += grad.get(illumination as f32) * MIRROR_COLOURING;

                let (n0, n1) = if hit.side().is_inside() {
                    (REF_INDEX, 1.0)
                } else {
                    (1.0, REF_INDEX)
                };
                let crossing = Crossing::new(ray.dir(), hit.side().norm(), n0, n1);
                if let Some(trans_dir) = crossing.trans_dir() {
                    let ref_prob = crossing.ref_prob();
                    let trans_prob = 1.0 - ref_prob;

                    let mut ref_ray = Ray::new(
                        *ray.pos(),
                        Crossing::init_ref_dir(
                            ray.dir(),
                            hit.side().norm(),
                            -ray.dir().dot(hit.side().norm()),
                        ),
                    );
                    ref_ray.travel(scene.sett().bump_dist());
                    let ref_col = colour(thread_id, scene, ref_ray, rng, weight * ref_prob);

                    let mut trans_ray = ray;
                    *trans_ray.dir_mut() = *trans_dir;
                    trans_ray.travel(scene.sett().bump_dist());
                    let trans_col = colour(thread_id, scene, trans_ray, rng, weight * trans_prob);

                    return col + (ref_col * ref_prob as f32) + (trans_col * trans_prob as f32);
                } else {
                    *ray.dir_mut() = *crossing.ref_dir();
                    ray.travel(scene.sett().bump_dist());
                }
            }
            _ => {
                let grad = Gradient::new(vec![
                    LinSrgba::default(),
                    scene.cols()[&hit.group()].get(x as f32),
                ]);
                col += grad.get(illumination as f32);
                sky = false;
                break;
            }
        }
    }

    if sky {
        return palette::Srgba::new(0.0, 0.0, (1.0 - ray.dir().z).powi(4) as f32, 1.0)
            .into_linear();
    }

    col
}

// /// Create a spatially varying value between zero and unity.
// #[inline]
// #[must_use]
// fn wobble(p: &Pos3) -> f64 {
//     (p.x.cos() * p.y.cos() * p.z.cos()).abs()
// }
