//! Simulation run functions.

use crate::{
    mcrt::{Data, Input},
    ParBar,
};
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

/// Run an MCRT simulation.
#[inline]
#[must_use]
pub fn simulate(input: &Input) -> Data {
    let pb = ParBar::new("Randomising", input.sett.num_phot());
    let pb = Arc::new(Mutex::new(pb));

    let threads: Vec<usize> = (0..num_cpus::get()).collect();
    let mut data: Vec<_> = threads
        .par_iter()
        .map(|id| single_thread(*id, &Arc::clone(&pb), &input))
        .collect();
    pb.lock()
        .expect("Could not lock progress bar.")
        .finish_with_message("Render complete");

    let mut base = data.pop().ok_or("Missing data result.").unwrap();
    for dat in data {
        base += &dat;
    }

    base
}

/// Simulate with a single thread.
#[inline]
fn single_thread(thread_id: usize, pb: &Arc<Mutex<ParBar>>, input: &Input) -> Data {
    let mut data = Data::new();

    while let Some((start, end)) = {
        let mut pb = pb.lock().expect("Could not lock progress bar.");
        let b = pb.block(input.sett.block_size());
        std::mem::drop(pb);
        b
    } {
        // data.emitted_photons += thread_id as f64;
        data.emitted_photons += 1.0;
    }

    data
}
