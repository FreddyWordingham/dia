//! Lighting functions.

use crate::{render::Scene, Crossing, Dir3, Hit, Ray};

/// Calculate the lighting factor.
#[inline]
#[must_use]
pub fn light(ray: &Ray, scene: &Scene, hit: &Hit) -> f64 {
    let light_dir = Dir3::new_normalize(scene.sett().sun_pos() - ray.pos());
    let view_dir = Dir3::new_normalize(scene.cam().focus().orient().pos() - ray.pos());
    let ref_dir = Crossing::init_ref_dir(
        ray.dir(),
        hit.side().norm(),
        -ray.dir().dot(hit.side().norm()),
    );

    let ambient = 1.0;
    let diffuse = hit.side().norm().dot(&light_dir).max(0.0);
    let specular = view_dir
        .dot(&ref_dir)
        .max(0.0)
        .powi(scene.sett().spec_pow());

    (ambient * scene.sett().light_weights()[0])
        + (diffuse * scene.sett().light_weights()[1])
        + (specular * scene.sett().light_weights()[2])
}
