//! Rendering simulation functions.

use crate::{
    render::{Input, Output, Painter},
    ParBar,
};
use minifb::{Scale, ScaleMode, Window, WindowOptions};
use palette::Pixel;
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
pub fn simulate(input: &Input, paint: Painter) -> Output {
    let num_pixels = input.cam.sensor().num_pixels();
    let width = input.cam.sensor().res().0 as usize;
    let height = input.cam.sensor().res().1 as usize;

    let buffer: Vec<u32> = vec![0; num_pixels as usize];
    let buffer = Arc::new(Mutex::new(buffer));

    let mut window = if input.sett.live() {
        Some({
            let mut win = Window::new(
                "DIA - Rendering",
                width,
                height,
                WindowOptions {
                    resize: true,
                    // scale: Scale::X4,
                    // scale: Scale::X2,
                    scale: Scale::X1,
                    scale_mode: ScaleMode::Center,
                    ..WindowOptions::default()
                },
            )
            .unwrap_or_else(|e| {
                panic!("{}", e);
            });
            win.update_with_buffer(&buffer.lock().unwrap(), width, height)
                .unwrap();
            win
        })
    } else {
        None
    };

    let pb = ParBar::new("Rendering", num_pixels as u64);
    let pb = Arc::new(Mutex::new(pb));

    // let mut rng = thread_rng();
    let data = Output::new(*input.grid.res(), [width, height]);
    let data = Arc::new(Mutex::new(data));

    while !pb.lock().unwrap().is_done() {
        let threads: Vec<usize> = (0..num_cpus::get()).collect();
        let _out: Vec<()> = threads
            .par_iter()
            .map(|_id| {
                cover_pix(
                    &Arc::clone(&pb),
                    &input,
                    &Arc::clone(&data),
                    &Arc::clone(&buffer),
                    paint,
                )
            })
            .collect();

        if input.sett.live() {
            window
                .as_mut()
                .unwrap()
                .update_with_buffer(&buffer.lock().unwrap(), width, height)
                .unwrap();
        }
    }

    pb.lock().unwrap().finish_with_message("Render complete");

    // Ok(Output::new(*input.grid.res(), [width, height]))
    if let Ok(d) = Arc::try_unwrap(data) {
        return d.into_inner().unwrap();
    }

    unreachable!("In theory...");
}

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

fn cover_pix(
    pb: &Arc<Mutex<ParBar>>,
    input: &Input,
    // mut rng: &mut ThreadRng,
    data: &Arc<Mutex<Output>>,
    buffer: &Arc<Mutex<Vec<u32>>>,
    paint: Painter,
) {
    let mut rng = thread_rng();

    if let Some((start, end)) = {
        let mut pb = pb.lock().unwrap();
        let b = pb.block(input.sett.block_size());
        std::mem::drop(pb);
        b
    } {
        for n in start..end {
            let pixel = (
                n % input.cam.sensor().res().0,
                n / input.cam.sensor().res().0,
            );

            for sub_sample in 0..input.cam.sensor().super_samples() {
                let offset = rng.gen_range(0.0, 2.0 * PI);
                for depth_sample in 0..input.cam.focus().dof().unwrap_or((1, 0.0)).0 {
                    let ray = input.cam.gen_ray(pixel, offset, sub_sample, depth_sample);
                    paint(
                        0,
                        &mut rng,
                        input,
                        data,
                        1.0 / f64::from(
                            input.cam.sensor().super_samples()
                                * input.cam.focus().dof().unwrap_or((1, 0.0)).0,
                        ),
                        [pixel.0 as usize, pixel.1 as usize],
                        ray,
                    );

                    let col: [u8; 4] = palette::Srgba::from_linear(
                        data.lock().unwrap().image[[pixel.0 as usize, pixel.1 as usize]],
                    )
                    .into_format()
                    .into_raw();
                    (buffer.lock().unwrap())
                        [(input.cam.sensor().num_pixels() - (n + 1)) as usize] =
                        from_u8_rgb(col[0], col[1], col[2]);
                }
            }
        }
    }
}
