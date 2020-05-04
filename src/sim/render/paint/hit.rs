//! Hit-scan painting function.

use crate::{
    render::{illumination, Scene},
    Ray,
};
use palette::LinSrgba;
use rand::rngs::ThreadRng;

/// Minimum fragment weight to simulate.
const MIN_WEIGHT: f64 = 0.01;

/// Colour if surface is hit.
#[inline]
#[must_use]
pub fn colour(
    _thread_id: usize,
    scene: &Scene,
    mut ray: Ray,
    _rng: &mut ThreadRng,
    weight: f64,
) -> LinSrgba {
    debug_assert!(weight > 0.0);

    let mut col = LinSrgba::default();

    if weight <= MIN_WEIGHT {
        return col;
    }

    if let Some(hit) = scene.grid().observe(ray.clone(), scene.sett().bump_dist()) {
        ray.travel(hit.dist());

        let light = illumination::light(&ray, scene, &hit);
        let shadow = illumination::shadow(&ray, scene, &hit);
        let illumination = light * shadow;

        match hit.group() {
            _ => {
                col += scene.cols()[&hit.group()].get(illumination as f32);
            }
        }
    } else {
        // Sky.
        return palette::Srgba::new(0.0, 0.0, (1.0 - ray.dir().z).powi(4) as f32, 1.0)
            .into_linear();
    }

    col
}