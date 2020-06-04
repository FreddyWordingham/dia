//! Rendering simulation functions.

use crate::{
    render::{Input, Output, Painter},
    Error, ParBar,
};
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

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
    _thread_id: usize,
    pb: &Arc<Mutex<ParBar>>,
    _input: &Input,
    _paint: Painter,
) -> Result<Output, Error> {
    let data = Output::new();
    Ok(data)
}
