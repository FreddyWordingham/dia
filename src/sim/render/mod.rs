//! Rendering module.

pub mod attribute;
pub mod camera;
pub mod settings;

pub use self::attribute::*;
pub use self::camera::*;
pub use self::settings::*;

use crate::{Adaptive, Error, Image, ParBar, Set};
use palette::{Gradient, LinSrgba};
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

/// Pixel block size.
const BLOCK_SIZE: u64 = 128;

/// Render an image.
/// # Errors
/// if an invalid thread image was created.
#[inline]
pub fn run(
    grid: &Adaptive,
    sett: &Settings,
    cam: &Camera,
    cols: &Set<Gradient<LinSrgba>>,
    attrs: &Set<Attribute>,
) -> Result<Image, Error> {
    let num_pixels = cam.sensor().num_pixels();
    let pb = ParBar::new("Rendering", num_pixels as u64);
    let pb = Arc::new(Mutex::new(pb));

    let threads: Vec<usize> = (0..num_cpus::get()).collect();
    let mut images: Vec<_> = threads
        .par_iter()
        .map(|id| run_thread(*id, &Arc::clone(&pb), grid, sett, cam, cols, attrs))
        .collect();
    pb.lock()?.finish_with_message("Render complete");

    let mut base = images.pop().unwrap()?;
    for img in images {
        base += &img?;
    }

    Ok(base)
}

/// Render on a single thread.
/// # Errors
/// if image creation fails.
#[inline]
fn run_thread(
    thread_id: usize,
    pb: &Arc<Mutex<ParBar>>,
    _grid: &Adaptive,
    _sett: &Settings,
    cam: &Camera,
    cols: &Set<Gradient<LinSrgba>>,
    _attrs: &Set<Attribute>,
) -> Result<Image, Error> {
    let mut img = Image::from_elem(cam.sensor().res(), LinSrgba::default());

    let hr_res = cam.sensor().res().0;

    while let Some((start, end)) = {
        let mut pb = pb.lock()?;
        let b = pb.block(BLOCK_SIZE);
        std::mem::drop(pb);
        b
    } {
        for n in start as usize..end as usize {
            let pixel = (n % hr_res, n / hr_res);

            img[pixel] += cols[&0].get(thread_id as f32 * 1.0 / 8.0);
        }
    }

    Ok(img)
}
