//! Rendering module.

pub mod camera;

pub use self::camera::*;

use crate::{Adaptive, Image, ParBar};
use num_cpus;
use palette::LinSrgba;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

/// Render an image.
#[inline]
#[must_use]
pub fn run(grid: &Adaptive, cam: &Camera) -> Image {
    let num_pixels = cam.sensor().num_pixels();
    let pb = ParBar::new("Rendering", num_pixels as u64);
    let pb = Arc::new(Mutex::new(pb));

    let threads: Vec<usize> = (0..num_cpus::get()).collect();
    let mut images: Vec<_> = threads
        .par_iter()
        .map(|id| run_thread(*id, grid, cam))
        .collect();
    pb.lock().unwrap().finish_with_message("Render complete");

    let mut base = images.pop().unwrap();
    for img in images {
        base += &img;
    }

    base
}

/// Render on a single thread.
#[inline]
#[must_use]
fn run_thread(_thread_id: usize, _grid: &Adaptive, cam: &Camera) -> Image {
    Image::from_elem(cam.sensor().res(), LinSrgba::default())
}
