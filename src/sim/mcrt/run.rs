//! Simulation run functions.

use crate::{
    mcrt::{life::Life, Data, Input},
    Error, ParBar,
};
use rand::thread_rng;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

/// Run an MCRT simulation.
/// # Errors
/// if the progress bar can not be locked.
#[inline]
pub fn simulate(input: &Input, func: Life) -> Result<Data, Error> {
    let pb = ParBar::new("Randomising", input.sett.num_phot());
    let pb = Arc::new(Mutex::new(pb));

    // let threads: Vec<usize> = (0..num_cpus::get()).collect(); // Multi-thread.
    let threads = vec![0]; // Single thread.
    let mut data: Vec<_> = threads
        .par_iter()
        .map(|id| single_thread(*id, &Arc::clone(&pb), input, func))
        .collect();
    pb.lock()?.finish_with_message("Render complete");

    let mut base = data.pop().ok_or("Missing data result.")??;
    for dat in data {
        base += &dat?;
    }

    Ok(base)
}

/// Simulate with a single thread.
/// # Errors
/// if the progress bar can not be locked.
#[inline]
fn single_thread(
    _thread_id: usize,
    pb: &Arc<Mutex<ParBar>>,
    input: &Input,
    func: Life,
) -> Result<Data, Error> {
    let mut data = Data::new(input.grid.boundary().clone(), *input.grid.res());

    let mut rng = thread_rng();

    while let Some((start, end)) = {
        let mut pb = pb.lock()?;
        let b = pb.block(input.sett.block_size());
        std::mem::drop(pb);
        b
    } {
        for _ in start..end {
            func(input, &mut data, &mut rng);
        }
    }

    Ok(data)
}
