//! Rendering simulation functions.

use crate::{
    render::{Input, Output, Painter},
    Error, ParBar,
};
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::{
    f64::consts::PI,
    sync::{Arc, Mutex},
};

/// Render an image.
/// # Errors
/// if an invalid thread image was created.
#[inline]
pub fn simulate(input: &Input, paint: Painter) -> Result<Output, Error> {
    let num_pixels = input.cam.sensor().num_pixels();
    let pb = ParBar::new("Rendering", num_pixels as u64);
    let pb = Arc::new(Mutex::new(pb));

    let threads: Vec<usize> = (0..num_cpus::get()).collect();
    let mut data: Vec<_> = threads
        .par_iter()
        .map(|id| single_thread(*id, &Arc::clone(&pb), input, paint))
        .collect();
    pb.lock()?.finish_with_message("Render complete");

    let mut output = data.pop().ok_or("Missing output result.")??;
    for d in data {
        output += &d?;
    }

    Ok(output)
}

/// Render on a single thread.
/// # Errors
/// if image creation fails.
#[inline]
fn single_thread(
    thread_id: usize,
    pb: &Arc<Mutex<ParBar>>,
    input: &Input,
    paint: Painter,
) -> Result<Output, Error> {
    let mut rng = thread_rng();

    let hr_res = input.cam.sensor().res().0;

    let super_samples = input.cam.sensor().super_samples();
    let dof_samples = input.cam.focus().dof().unwrap_or((1, 0.0)).0;
    let weight = 1.0 / f64::from(super_samples * dof_samples);

    let mut data = Output::new(
        *input.grid.res(),
        [
            input.cam.sensor().res().0 as usize,
            input.cam.sensor().res().1 as usize,
        ],
    );

    while let Some((start, end)) = {
        let mut pb = pb.lock()?;
        let b = pb.block(input.sett.block_size());
        std::mem::drop(pb);
        b
    } {
        for n in start..end {
            let pixel = (n % hr_res, n / hr_res);

            for sub_sample in 0..super_samples {
                let offset = rng.gen_range(0.0, 2.0 * PI);
                for depth_sample in 0..dof_samples {
                    let ray = input.cam.gen_ray(pixel, offset, sub_sample, depth_sample);
                    paint(
                        thread_id,
                        &mut rng,
                        input,
                        &mut data,
                        weight,
                        [pixel.0 as usize, pixel.1 as usize],
                        ray,
                    );
                }
            }
        }
    }

    Ok(data)
}
