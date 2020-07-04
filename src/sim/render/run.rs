//! Rendering simulation functions.

use crate::{
    render::{Input, Output, Painter},
    Error, ParBar,
};
use minifb::{Scale, ScaleMode, Window, WindowOptions};
use palette::Pixel;
use rand::{rngs::ThreadRng, thread_rng, Rng};
use rayon::prelude::*;
use std::{
    f64::consts::PI,
    sync::{Arc, Mutex},
};

// /// Render an image.
// /// # Errors
// /// if an invalid thread image was created.
// #[inline]
// pub fn simulate(input: &Input, paint: Painter) -> Result<Output, Error> {
//     let num_pixels = input.cam.sensor().num_pixels();
//     let pb = ParBar::new("Rendering", num_pixels as u64);
//     let pb = Arc::new(Mutex::new(pb));

//     let threads: Vec<usize> = (0..num_cpus::get()).collect();
//     let mut data: Vec<_> = threads
//         .par_iter()
//         .map(|id| single_thread(*id, &Arc::clone(&pb), input, paint))
//         .collect();
//     pb.lock()?.finish_with_message("Render complete");

//     let mut output = data.pop().ok_or("Missing output result.")??;
//     for d in data {
//         output += &d?;
//     }

//     Ok(output)
// }

// /// Render an image.
// /// # Errors
// /// if an invalid thread image was created.
// #[inline]
// pub fn simulate(input: &Input, paint: Painter) -> Result<Output, Error> {
//     let num_pixels = input.cam.sensor().num_pixels();

//     let mut buffer: Vec<u32> = vec![0; num_pixels as usize];
//     let width = input.cam.sensor().res().0 as usize;
//     let height = input.cam.sensor().res().1 as usize;

//     let mut window = Window::new(
//         "DIA - Rendering",
//         width,
//         height,
//         WindowOptions {
//             resize: true,
//             // scale: Scale::X4,
//             scale: Scale::X2,
//             // scale: Scale::X1,
//             scale_mode: ScaleMode::Center,
//             ..WindowOptions::default()
//         },
//     )
//     .unwrap_or_else(|e| {
//         panic!("{}", e);
//     });
//     window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
//     window.update_with_buffer(&buffer, width, height).unwrap();

//     let mut rng = thread_rng();

//     let hr_res = input.cam.sensor().res().0;

//     let super_samples = input.cam.sensor().super_samples();
//     let dof_samples = input.cam.focus().dof().unwrap_or((1, 0.0)).0;
//     let weight = 1.0 / f64::from(super_samples * dof_samples);

//     let mut data = Output::new(*input.grid.res(), [width, height]);

//     let mut block = 0;
//     for n in 0..num_pixels {
//         block += 1;
//         let pixel = (n % hr_res, n / hr_res);

//         for sub_sample in 0..super_samples {
//             let offset = rng.gen_range(0.0, 2.0 * PI);
//             for depth_sample in 0..dof_samples {
//                 let ray = input.cam.gen_ray(pixel, offset, sub_sample, depth_sample);
//                 paint(
//                     0,
//                     &mut rng,
//                     input,
//                     &mut data,
//                     weight,
//                     [pixel.0 as usize, pixel.1 as usize],
//                     ray,
//                 );
//                 let col: [u8; 4] =
//                     palette::Srgba::from_linear(data.image[[pixel.0 as usize, pixel.1 as usize]])
//                         .into_format()
//                         .into_raw();
//                 buffer[(num_pixels - (n + 1)) as usize] = from_u8_rgb(col[0], col[1], col[2]);

//                 if block > input.sett.block_size() {
//                     block = 0;
//                     window
//                         .update_with_buffer(&buffer, width, height)
//                         .expect("Failed to updated window buffer.");
//                 }
//             }
//         }
//     }

//     Ok(data)
// }

/// Render an image.
/// # Errors
/// if an invalid thread image was created.
#[inline]
pub fn simulate(input: &Input, paint: Painter) -> Result<Output, Error> {
    let num_pixels = input.cam.sensor().num_pixels();
    let width = input.cam.sensor().res().0 as usize;
    let height = input.cam.sensor().res().1 as usize;

    let mut buffer: Vec<u32> = vec![0; num_pixels as usize];
    let buffer = Arc::new(Mutex::new(buffer));
    let mut window = Window::new(
        "DIA - Rendering",
        width,
        height,
        WindowOptions {
            resize: true,
            // scale: Scale::X4,
            scale: Scale::X2,
            // scale: Scale::X1,
            scale_mode: ScaleMode::Center,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    window
        .update_with_buffer(&buffer.lock().unwrap(), width, height)
        .unwrap();

    let pb = ParBar::new("Rendering", num_pixels as u64);
    let pb = Arc::new(Mutex::new(pb));

    let hr_res = input.cam.sensor().res().0;
    let super_samples = input.cam.sensor().super_samples();
    let dof_samples = input.cam.focus().dof().unwrap_or((1, 0.0)).0;
    let weight = 1.0 / f64::from(super_samples * dof_samples);

    // let mut rng = thread_rng();
    let mut data = Output::new(*input.grid.res(), [width, height]);
    let mut data = Arc::new(Mutex::new(data));

    while !pb.lock().unwrap().is_done() {
        let threads: Vec<usize> = (0..num_cpus::get()).collect();
        let _out: Vec<()> = threads
            .par_iter()
            .map(|id| {
                cover_pix(
                    &Arc::clone(&pb),
                    &input,
                    &Arc::clone(&data),
                    &Arc::clone(&buffer),
                    paint,
                )
            })
            .collect();

        window
            .update_with_buffer(&buffer.lock().unwrap(), width, height)
            .unwrap();
    }

    pb.lock()?.finish_with_message("Render complete");

    // Ok(data)
    Ok(Output::new(*input.grid.res(), [width, height]))
}

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

fn cover_pix(
    pb: &Arc<Mutex<ParBar>>,
    input: &Input,
    // mut rng: &mut ThreadRng,
    mut data: &Arc<Mutex<Output>>,
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

// /// Render on a single thread.
// /// # Errors
// /// if image creation fails.
// #[inline]
// fn single_thread(
//     window: &Arc<Mutex<Window>>,
//     buffer: &Arc<Mutex<Vec<u32>>>,
//     thread_id: usize,
//     pb: &Arc<Mutex<ParBar>>,
//     input: &Input,
//     paint: Painter,
// ) -> Result<Output, Error> {
//     let mut rng = thread_rng();

//     let hr_res = input.cam.sensor().res().0;

//     let super_samples = input.cam.sensor().super_samples();
//     let dof_samples = input.cam.focus().dof().unwrap_or((1, 0.0)).0;
//     let weight = 1.0 / f64::from(super_samples * dof_samples);

//     let mut data = Output::new(
//         *input.grid.res(),
//         [
//             input.cam.sensor().res().0 as usize,
//             input.cam.sensor().res().1 as usize,
//         ],
//     );

//     while let Some((start, end)) = {
//         let mut pb = pb.lock()?;
//         let b = pb.block(input.sett.block_size());
//         std::mem::drop(pb);
//         b
//     } {
//         for n in start..end {
//             let pixel = (n % hr_res, n / hr_res);

//             for sub_sample in 0..super_samples {
//                 let offset = rng.gen_range(0.0, 2.0 * PI);
//                 for depth_sample in 0..dof_samples {
//                     let ray = input.cam.gen_ray(pixel, offset, sub_sample, depth_sample);
//                     paint(
//                         thread_id,
//                         &mut rng,
//                         input,
//                         &mut data,
//                         weight,
//                         [pixel.0 as usize, pixel.1 as usize],
//                         ray,
//                     );
//                 }
//             }
//         }
//     }

//     Ok(data)
// }
