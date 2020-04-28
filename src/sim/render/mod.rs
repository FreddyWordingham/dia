//! Rendering module.

pub mod camera;

pub use self::camera::*;

use crate::{Adaptive, Image, ParBar};
use num_cpus;
use palette::{Gradient, LinSrgba};
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

/// Pixel block size.
const BLOCK_SIZE: u64 = 128;

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
        .map(|id| run_thread(*id, &Arc::clone(&pb), grid, cam))
        .collect();
    pb.lock()
        .expect("Unable to lock progress bar.")
        .finish_with_message("Render complete");

    let mut base = images.pop().unwrap();
    for img in images {
        base += &img;
    }

    base
}

/// Render on a single thread.
#[inline]
#[must_use]
fn run_thread(thread_id: usize, pb: &Arc<Mutex<ParBar>>, _grid: &Adaptive, cam: &Camera) -> Image {
    let mut img = Image::from_elem(cam.sensor().res(), LinSrgba::default());

    let backup = Gradient::new(vec![
        LinSrgba::new(0.0, 0.0, 0.0, 1.0),
        LinSrgba::new(1.0, 1.0, 1.0, 1.0),
    ]);

    let hr_res = cam.sensor().res().0;

    while let Some((start, end)) = {
        let mut pb = pb.lock().expect("Could not lock progress bar.");
        let b = pb.block(BLOCK_SIZE);
        std::mem::drop(pb);
        b
    } {
        for n in start as usize..end as usize {
            let pixel = (n % hr_res, n / hr_res);

            img[pixel] += backup.get(thread_id as f32 * 1.0 / 8.0);
        }
    }

    img
}
