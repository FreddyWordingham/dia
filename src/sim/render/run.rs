//! Rendering simulation run functions.

use crate::{
    render::{paint, Scene},
    Error, Image, ParBar,
};
use palette::LinSrgba;
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::{
    f64::consts::PI,
    sync::{Arc, Mutex},
};

/// Pixel block size.
const BLOCK_SIZE: u64 = 12;

/// Render an image.
/// # Errors
/// if an invalid thread image was created.
#[inline]
pub fn capture(scene: &Scene) -> Result<Image, Error> {
    let num_pixels = scene.cam().sensor().num_pixels();
    let pb = ParBar::new("Rendering", num_pixels as u64);
    let pb = Arc::new(Mutex::new(pb));

    let threads: Vec<usize> = (0..num_cpus::get()).collect();
    let mut images: Vec<_> = threads
        .par_iter()
        .map(|id| single_thread(*id, &Arc::clone(&pb), scene))
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
fn single_thread(thread_id: usize, pb: &Arc<Mutex<ParBar>>, scene: &Scene) -> Result<Image, Error> {
    let cam = scene.cam();

    let mut img = Image::from_elem(cam.sensor().res(), LinSrgba::default());
    let mut rng = thread_rng();

    let hr_res = cam.sensor().res().0;

    let super_samples = cam.sensor().super_samples();
    let dof_samples = cam.focus().dof().unwrap_or((1, 0.0)).0;
    let weight = 1.0 / (super_samples * dof_samples) as f32;

    while let Some((start, end)) = {
        let mut pb = pb.lock()?;
        let b = pb.block(BLOCK_SIZE);
        std::mem::drop(pb);
        b
    } {
        for n in start as usize..end as usize {
            let pixel = (n % hr_res, n / hr_res);

            for sub_sample in 0..super_samples {
                let offset = rng.gen_range(0.0, 2.0 * PI);
                for depth_sample in 0..dof_samples {
                    let ray = cam.gen_ray(pixel, offset, sub_sample, depth_sample);
                    img[pixel] += paint::hit::colour(thread_id, scene, ray, &mut rng, 1.0) * weight;
                }
            }
        }
    }

    Ok(img)
}
