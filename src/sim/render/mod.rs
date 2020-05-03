//! Rendering module.

pub mod attribute;
pub mod camera;
pub mod settings;

pub use self::attribute::*;
pub use self::camera::*;
pub use self::settings::*;

use crate::{Adaptive, Error, Image, ParBar, Ray, Set};
use palette::{Gradient, LinSrgba};
use rand::rngs::ThreadRng;
use rand::thread_rng;
use rand::Rng;
use rayon::prelude::*;
use std::f64::consts::PI;
use std::sync::{Arc, Mutex};

/// Pixel block size.
const BLOCK_SIZE: u64 = 12;

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
    grid: &Adaptive,
    sett: &Settings,
    cam: &Camera,
    cols: &Set<Gradient<LinSrgba>>,
    attrs: &Set<Attribute>,
) -> Result<Image, Error> {
    let mut img = Image::from_elem(cam.sensor().res(), LinSrgba::default());
    let mut rng = thread_rng();

    let hr_res = cam.sensor().res().0;

    let super_samples = cam.sensor().super_samples();
    let dof_samples = cam.focus().dof().unwrap_or((1, 0.0)).0;

    while let Some((start, end)) = {
        let mut pb = pb.lock()?;
        let b = pb.block(BLOCK_SIZE);
        std::mem::drop(pb);
        b
    } {
        for n in start as usize..end as usize {
            let pixel = (n % hr_res, n / hr_res);

            let offset = rng.gen_range(0.0, 2.0 * PI); // TODO: Try placing within super sample?
            for sub_sample in 0..super_samples {
                for depth_sample in 0..dof_samples {
                    let ray = cam.gen_ray(pixel, offset, sub_sample, depth_sample);
                    img[pixel] += paint(thread_id, ray, grid, sett, cam, cols, attrs, &mut rng);
                }
            }
        }
    }

    Ok(img)
}

/// Determine the colour of a given observation ray.
#[inline]
#[must_use]
fn paint(
    _thread_id: usize,
    ray: Ray,
    grid: &Adaptive,
    sett: &Settings,
    _cam: &Camera,
    cols: &Set<Gradient<LinSrgba>>,
    _attrs: &Set<Attribute>,
    _rng: &mut ThreadRng,
) -> LinSrgba {
    let mut col = LinSrgba::default();
    if let Some(hit) = grid.observe(ray, sett.bump_dist()) {
        col += cols[&0].get(hit.dist() as f32 / 10.0);
    }

    col
}
