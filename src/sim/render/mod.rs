//! Rendering module.

pub mod camera;

pub use self::camera::*;

use crate::Image;

/// Render an image.
#[inline]
#[must_use]
pub fn run() -> Image {}
