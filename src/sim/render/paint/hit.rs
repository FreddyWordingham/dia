//! Hit-scan painting function.

use crate::{render::Scene, Hit, Ray};
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
        col += scene.cols()[&0].get(hit.dist() as f32 / 10.0);
        ray.travel(hit.dist());

        let light = light(&ray, scene, &hit);
        let shadow = shadow(&ray, scene, &hit);
        let illumination = light * shadow;

        match hit.group() {
            _ => {
                col += scene.cols()[&hit.group()].get(1.0);
            }
        }
    } else {
        // Sky.
        return palette::Srgba::new(0.0, 0.0, (1.0 - ray.dir().z).powi(4) as f32, 1.0)
            .into_linear();
    }

    col
}

/// Calculate the lighting. 0.0 = Complete darkness. 1.0 = Full brightness.
#[inline]
#[must_use]
fn light(_ray: &Ray, _scene: &Scene, _hit: &Hit) -> f64 {
    0.9
}

/// Calculate the shadowing multiplier. 0.0 = Full shadow. 1.0 = No shadow.
#[inline]
#[must_use]
fn shadow(_ray: &Ray, _scene: &Scene, _hit: &Hit) -> f64 {
    0.3
}
