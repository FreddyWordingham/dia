//! Simulation run functions.

use crate::{mcrt::Data, ParBar};
use rayon::prelude::*;
use std::{
    f64::consts::PI,
    sync::{Arc, Mutex},
};

/// Run an MCRT simulation.
#[inline]
#[must_use]
pub fn simulate(input: &Input) -> Data {
    let pb = ParBar::new("Randomising", input.sett().num_phot());
    let pb = Arc::new(Mutex::new(pb));

    let threads: Vec<usize> = (0..num_cpus::get()).collect();
    let mut images: Vec<_> = threads
        .par_iter()
        .map(|id| single_thread(*id, &Arc::clone(&pb)))
        .collect();
    pb.lock()?.finish_with_message("Render complete");
}

/// Simulate with a single thread.
#[inline]
fn single_thread(thread_id: usize, pb: &Arc<Mutex<ParBar>>) -> Data {
    let mut data = Data::new();

    data += thread_id as f64;

    data
}
