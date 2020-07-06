//! Rendering simulation functions.

use crate::{
    render::{painter, Input, Output, Scene},
    Error, ParBar,
};
use minifb::{CursorStyle, Scale, ScaleMode, Window, WindowOptions};
use palette::Pixel;
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::{
    f64::consts::PI,
    sync::{Arc, Mutex},
};

/// Render an image with a live preview.
/// # Errors
/// if window could not be constructed or
/// a mutex unwrapping failed or
/// an arc unwrapping failed.
#[inline]
pub fn simulate_live(input: &Input, scene: &Scene) -> Result<Output, Error> {
    let num_pixels = scene.cam().sensor().num_pixels();
    let width = scene.cam().sensor().res().0 as usize;
    let height = scene.cam().sensor().res().1 as usize;

    let buffer: Vec<u32> = vec![0; num_pixels as usize];
    let buffer = Arc::new(Mutex::new(buffer));
    let mut win = Window::new(
        "Rendering",
        width,
        height,
        WindowOptions {
            resize: true,
            scale: Scale::FitScreen,
            scale_mode: ScaleMode::AspectRatioStretch,
            ..WindowOptions::default()
        },
    )?;
    win.set_cursor_style(CursorStyle::Crosshair);
    win.update_with_buffer(&buffer.lock()?, width, height)?;

    let mut main_bar = ParBar::new("Rendering", num_pixels as u64);

    let data = Output::new([width, height]);
    let data = Arc::new(Mutex::new(data));

    let threads: Vec<usize> = (0..num_cpus::get()).collect();
    if let Some((start, end)) = main_bar.block(input.sett.block_size()) {
        let pb = ParBar::new("Rendering Block", end - start);
        let pb = Arc::new(Mutex::new(pb));

        let _out: Vec<()> = threads
            .par_iter()
            .map(|_id| {
                render_pix(
                    start,
                    &Arc::clone(&pb),
                    input,
                    scene,
                    &Arc::clone(&data),
                    &Arc::clone(&buffer),
                )
            })
            .collect();

        win.update_with_buffer(&buffer.lock()?, width, height)?;
    }
    main_bar.finish_with_message("Render complete.");

    if let Ok(d) = Arc::try_unwrap(data) {
        return Ok(d.into_inner()?);
    }

    unreachable!("Failed to unwrap data.");
}

/// Render a range of pixels using a single thread.
#[allow(clippy::result_expect_used)]
#[inline]
fn render_pix(
    buffer_start: u64,
    pb: &Arc<Mutex<ParBar>>,
    input: &Input,
    scene: &Scene,
    data: &Arc<Mutex<Output>>,
    buffer: &Arc<Mutex<Vec<u32>>>,
) {
    let mut rng = thread_rng();
    let super_samples = scene.cam().sensor().super_samples();
    let dof_samples = scene.cam().focus().dof_samples();
    let h_res = scene.cam().sensor().res().0;
    let total_pixels = scene.cam().sensor().num_pixels();

    let weight = 1.0 / (super_samples * dof_samples) as f32;

    if let Some((start, end)) = {
        let mut pb = pb.lock().expect("Could not lock progress bar.");
        let b = pb.block(input.sett.sub_block_size());
        std::mem::drop(pb);
        b
    } {
        for mut n in start..end {
            n += buffer_start;
            let pixel = [(n % h_res) as usize, (n / h_res) as usize];
            let mut col = palette::LinSrgba::default();

            for sub_sample in 0..super_samples {
                let offset = rng.gen_range(0.0, 2.0 * PI);
                for depth_sample in 0..dof_samples {
                    let ray = scene.cam().gen_ray(pixel, offset, sub_sample, depth_sample);
                    col += painter::test(&mut rng, input, scene, ray, 0) * weight;
                }
            }

            let raw_col: [u8; 4] = palette::Srgba::from_linear(col).into_format().into_raw();

            buffer.lock().expect("Could not lock window buffer.")
                [(total_pixels - (n + 1)) as usize] =
                from_u8_rgb(raw_col[0], raw_col[1], raw_col[2]);
            data.lock().expect("Could not lock data.").image[pixel] = col;
        }
    }
}

/// Create a 32 bit colour representation from 8 bit components.
#[inline]
#[must_use]
const fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}
