//! Rendering simulation functions.

use crate::{
    render::{Engine, Input, Order, Output, Scene},
    Bar, Error, SilentBar, BLUE, GREEN, RED,
};
use minifb::{CursorStyle, Scale, ScaleMode, Window, WindowOptions};
use palette::Pixel;
use rand::{seq::SliceRandom, thread_rng, Rng};
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
pub fn simulate_live(engine: Engine, input: &Input, scene: &Scene) -> Result<Output, Error> {
    let num_pixels = scene.cam().sensor().num_pixels();
    let width = scene.cam().sensor().res().0 as usize;
    let height = scene.cam().sensor().res().1 as usize;

    let order: Vec<u64> = match input.sett.order() {
        Order::Forward => (0..num_pixels).collect(),
        Order::Backward => {
            let mut o: Vec<u64> = (0..num_pixels).collect();
            o.reverse();
            o
        }
        Order::Shuffle => {
            let mut o: Vec<u64> = (0..num_pixels).collect();
            o.shuffle(&mut thread_rng());
            o
        }
    };

    let img_buffer: Vec<u32> = vec![0; num_pixels as usize];
    let img_buffer = Arc::new(Mutex::new(img_buffer));
    let mut img_win = Window::new(
        "Rendering in Space",
        width,
        height,
        WindowOptions {
            resize: true,
            scale: Scale::FitScreen,
            scale_mode: ScaleMode::AspectRatioStretch,
            ..WindowOptions::default()
        },
    )?;
    img_win.set_cursor_style(CursorStyle::Crosshair);
    img_win.update_with_buffer(&img_buffer.lock()?, width, height)?;

    let time_buffer: Vec<u32> = vec![0; num_pixels as usize];
    let time_buffer = Arc::new(Mutex::new(time_buffer));
    let mut time_win = Window::new(
        "Rendering in Time",
        width,
        height,
        WindowOptions {
            resize: true,
            scale: Scale::FitScreen,
            scale_mode: ScaleMode::AspectRatioStretch,
            ..WindowOptions::default()
        },
    )?;
    time_win.set_cursor_style(CursorStyle::Crosshair);
    time_win.update_with_buffer(&time_buffer.lock()?, width, height)?;

    let mut main_bar = Bar::new("Rendering", num_pixels as u64);

    let data = Output::new([width, height]);
    let data = Arc::new(Mutex::new(data));

    let threads: Vec<usize> = (0..num_cpus::get()).collect();
    while let Some((start, end)) = main_bar.block(input.sett.block_size()) {
        let pb = SilentBar::new(end - start);
        let pb = Arc::new(Mutex::new(pb));

        while !pb.lock()?.is_done() {
            let _out: Vec<()> = threads
                .par_iter()
                .map(|_id| {
                    render_pix(
                        engine,
                        &order,
                        start,
                        &Arc::clone(&pb),
                        input,
                        scene,
                        &Arc::clone(&data),
                        [&Arc::clone(&img_buffer), &Arc::clone(&time_buffer)],
                    )
                })
                .collect();
        }

        img_win.update_with_buffer(&img_buffer.lock()?, width, height)?;
        time_win.update_with_buffer(&time_buffer.lock()?, width, height)?;
    }
    main_bar.finish_with_message("Render complete.");

    if let Ok(d) = Arc::try_unwrap(data) {
        return Ok(d.into_inner()?);
    }

    unreachable!("Failed to unwrap data.");
}

/// Render an image without a preview.
/// # Errors
/// if a mutex unwrapping failed or
/// an arc unwrapping failed.
#[inline]
pub fn simulate_bts(engine: Engine, input: &Input, scene: &Scene) -> Result<Output, Error> {
    let num_pixels = scene.cam().sensor().num_pixels();
    let width = scene.cam().sensor().res().0 as usize;
    let height = scene.cam().sensor().res().1 as usize;

    let main_bar = Bar::new("Rendering", num_pixels as u64);
    let main_bar = Arc::new(Mutex::new(main_bar));

    let data = Output::new([width, height]);
    let data = Arc::new(Mutex::new(data));

    let threads: Vec<usize> = (0..num_cpus::get()).collect();
    while !main_bar.lock()?.is_done() {
        let _out: Vec<()> = threads
            .par_iter()
            .map(|_id| {
                render_pix_lin(
                    engine,
                    &Arc::clone(&main_bar),
                    input,
                    scene,
                    &Arc::clone(&data),
                )
            })
            .collect();
    }
    main_bar.lock()?.finish_with_message("Render complete.");

    if let Ok(d) = Arc::try_unwrap(data) {
        return Ok(d.into_inner()?);
    }

    unreachable!("Failed to unwrap data.");
}

/// Render a range of pixels using a single thread.
#[allow(clippy::result_expect_used)]
#[inline]
fn render_pix_lin(
    engine: Engine,
    pb: &Arc<Mutex<Bar>>,
    input: &Input,
    scene: &Scene,
    data: &Arc<Mutex<Output>>,
) {
    let mut rng = thread_rng();
    let super_samples = scene.cam().sensor().super_samples();
    let dof_samples = scene.cam().focus().dof_samples();
    let h_res = scene.cam().sensor().res().0;

    let weight = 1.0 / f64::from(super_samples * dof_samples);

    if let Some((start, end)) = {
        let mut pb = pb.lock().expect("Could not lock progress bar.");
        let b = pb.block(input.sett.sub_block_size());
        std::mem::drop(pb);
        b
    } {
        for p in start..end {
            let now = std::time::Instant::now();

            let pixel = [(p % h_res) as usize, (p / h_res) as usize];
            let mut col = palette::LinSrgba::default();

            for sub_sample in 0..super_samples {
                let offset = rng.gen_range(0.0, 2.0 * PI);
                for depth_sample in 0..dof_samples {
                    let ray = scene.cam().gen_ray(pixel, offset, sub_sample, depth_sample);
                    col += engine(&mut rng, input, scene, ray, 1.0) * weight as f32;
                }
            }

            let time = std::time::Instant::now().duration_since(now).as_nanos();
            let t = time_scaler(weight * time as f64);
            let time_col = input.cols.map()["time"].get(t as f32);
            data.lock().expect("Could not lock data.").time[pixel] = time_col;
            data.lock().expect("Could not lock data.").image[pixel] = col;
        }
    }
}

/// Render a range of pixels using a single thread.
#[allow(clippy::result_expect_used)]
#[inline]
fn render_pix(
    engine: Engine,
    order: &[u64],
    buffer_start: u64,
    pb: &Arc<Mutex<SilentBar>>,
    input: &Input,
    scene: &Scene,
    data: &Arc<Mutex<Output>>,
    buffers: [&Arc<Mutex<Vec<u32>>>; 2],
) {
    let [img_buffer, time_buffer] = buffers;

    let mut rng = thread_rng();
    let super_samples = scene.cam().sensor().super_samples();
    let dof_samples = scene.cam().focus().dof_samples();
    let h_res = scene.cam().sensor().res().0;
    let total_pixels = scene.cam().sensor().num_pixels();

    let weight = 1.0 / f64::from(super_samples * dof_samples);

    if let Some((start, end)) = {
        let mut pb = pb.lock().expect("Could not lock progress bar.");
        let b = pb.block(input.sett.sub_block_size());
        std::mem::drop(pb);
        b
    } {
        for q in start..end {
            let now = std::time::Instant::now();

            let p = order[(q + buffer_start) as usize];
            let pixel = [(p % h_res) as usize, (p / h_res) as usize];
            let mut col = palette::LinSrgba::default();

            for sub_sample in 0..super_samples {
                let offset = rng.gen_range(0.0, 2.0 * PI);
                for depth_sample in 0..dof_samples {
                    let ray = scene.cam().gen_ray(pixel, offset, sub_sample, depth_sample);
                    col += engine(&mut rng, input, scene, ray, 1.0) * weight as f32;
                }
            }

            let time = std::time::Instant::now().duration_since(now).as_nanos();
            let t = time_scaler(weight * time as f64);
            let time_col = input.cols.map()["time"].get(t as f32);
            data.lock().expect("Could not lock data.").time[pixel] = time_col;
            let raw_time_col: [u8; 4] = time_col.into_format().into_raw();
            time_buffer
                .lock()
                .expect("Could not lock the time window buffer.")
                [(total_pixels - (p + 1)) as usize] =
                from_u8_rgb(raw_time_col[RED], raw_time_col[GREEN], raw_time_col[BLUE]);

            data.lock().expect("Could not lock data.").image[pixel] = col;
            let raw_col: [u8; 4] = col.into_format().into_raw();
            img_buffer.lock().expect("Could not lock window buffer.")
                [(total_pixels - (p + 1)) as usize] =
                from_u8_rgb(raw_col[RED], raw_col[GREEN], raw_col[BLUE]);
        }
    }
}

/// Create a 32 bit colour representation from 8 bit components.
#[inline]
#[must_use]
const fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

/// Scale a time in nano seconds to between zero and unity.
#[inline]
#[must_use]
fn time_scaler(time: f64) -> f64 {
    (time.log10() * 0.1).min(1.0)
}
