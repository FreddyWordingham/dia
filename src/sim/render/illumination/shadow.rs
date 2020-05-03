//! Shadowing functions.

use crate::{render::Scene, Hit, Ray};

/// Calculate the shadowing factor.
#[inline]
#[must_use]
pub fn shadow(_ray: &Ray, _scene: &Scene, _hit: &Hit) -> f64 {
    0.5
}
