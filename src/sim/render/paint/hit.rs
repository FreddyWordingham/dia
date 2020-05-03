//! Hit-scan painting function.

use crate::{render::Scene, Ray};
use palette::LinSrgba;
use rand::rngs::ThreadRng;

/// Colour if surface is hit.
#[inline]
#[must_use]
pub fn colour(_thread_id: usize, scene: &Scene, ray: Ray, _rng: &mut ThreadRng) -> LinSrgba {
    let grid = scene.grid();
    let sett = scene.sett();
    let _cam = scene.cam();
    let cols = scene.cols();
    let _attrs = scene.attrs();

    let mut col = LinSrgba::default();
    if let Some(hit) = grid.observe(ray, sett.bump_dist()) {
        col += cols[&0].get(hit.dist() as f32 / 10.0);
    }

    col
}
