//! Lighting functions.

use crate::{Crossing, Dir3, Hit, Pos3, Ray};

/// Ambient lighting weighting.
const AMBIENT_WEIGHT: f64 = 0.3;

/// Diffuse lighting weighting.
const DIFFUSE_WEIGHT: f64 = 0.5;

/// Specular lighting weighting.
const SPECULAR_WEIGHT: f64 = 0.2;

/// Specular lighting power.
const SPECULAR_POWER: i32 = 8;

/// Calculate the lighting factor.
#[inline]
#[must_use]
pub fn light(sun_pos: &Pos3, cam_pos: &Pos3, ray: &Ray, hit: &Hit) -> f64 {
    let light_dir = Dir3::new_normalize(sun_pos - ray.pos());
    let view_dir = Dir3::new_normalize(cam_pos - ray.pos());
    let ref_dir = Crossing::init_ref_dir(
        ray.dir(),
        hit.side().norm(),
        -ray.dir().dot(hit.side().norm()),
    );

    let mut ambient = 1.0;
    let mut diffuse = hit.side().norm().dot(&light_dir).max(0.0);
    let mut specular = view_dir.dot(&ref_dir).max(0.0).powi(SPECULAR_POWER);

    ambient *= AMBIENT_WEIGHT;
    diffuse *= DIFFUSE_WEIGHT;
    specular *= SPECULAR_WEIGHT;

    ambient + diffuse + specular
}
