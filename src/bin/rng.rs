//! Formula testing binary.

use attr::input;
use dia::*;
use rand::thread_rng;
use rayon::prelude::*;
use std::{
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

/// Input parameters.
#[input]
struct Parameters {
    /// Number of sample to take of the distribution.
    samples: u64,
    /// Block size.
    block_size: u64,
    /// Histogram minimum bound.
    min: f64,
    /// Histogram maximum bound.
    max: f64,
    /// Number of bins.
    bins: u64,
    /// Probability distribution.
    dist: form::Probability,
}

/// Runtime parameters.
pub struct Input {
    /// Number of sample to take of the distribution.
    pub samples: u64,
    /// Block size.
    pub block_size: u64,
    /// Histogram minimum bound.
    pub min: f64,
    /// Histogram maximum bound.
    pub max: f64,
    /// Number of bins.
    pub bins: u64,
    /// Probability distribution.
    pub dist: Probability,
}

/// Main function.
pub fn main() {
    banner::title("RNG Testing");
    let (params_path, in_dir, out_dir) = init();
    let params = input(&in_dir, &params_path);
    let input = build(&in_dir, params);
    let data = simulate(&input);
    save(&out_dir, data);
    banner::section("Finished");
}

/// Initialise the command line arguments and directories.
fn init() -> (PathBuf, PathBuf, PathBuf) {
    banner::section("Initialisation");
    banner::sub_section("Command line args");
    args!(
        bin_path: PathBuf //  ; params_path: PathBuf
                          //
    );
    let params_path = Path::new("parameters.json5").to_path_buf();
    report!("binary path", bin_path.display());
    report!("parameters path", params_path.display());

    banner::sub_section("Directories");
    let (in_dir, out_dir) = dir::io_dirs(None, None).expect("Could not initialise directories");
    report!("input directory", in_dir.display());
    report!("output directory", out_dir.display());

    (params_path, in_dir, out_dir)
}

/// Load the input files.
fn input(in_dir: &Path, params_path: &Path) -> Parameters {
    banner::section("Input");
    banner::sub_section("Parameters");
    let path = in_dir.join(params_path);

    Parameters::load(&path).expect("Could not load parameters file")
}

/// Build instances.
fn build(in_dir: &Path, params: Parameters) -> Input {
    banner::section("Building");

    let samples = params.samples;
    let block_size = params.block_size;
    let min = params.min;
    let max = params.max;
    let bins = params.bins;
    let dist = params.dist.build(in_dir).expect("Could not build formula.");

    Input {
        samples,
        block_size,
        min,
        max,
        bins,
        dist,
    }
}

/// Run the simulation.
fn simulate(input: &Input) -> Histogram {
    banner::section("Simulating");

    let pb = ParBar::new("Randomising", input.samples);
    let pb = Arc::new(Mutex::new(pb));

    let threads: Vec<usize> = (0..num_cpus::get()).collect(); // Multi-thread.
                                                              // let threads = vec![0]; // Single thread.
    let mut data: Vec<_> = threads
        .par_iter()
        .map(|id| single_thread(*id, &Arc::clone(&pb), input))
        .collect();
    pb.lock()
        .expect("Unable to lock progress bar.")
        .finish_with_message("Randomisation complete");

    let mut base = data.pop().ok_or("Missing data result.").unwrap();
    for dat in data {
        base += &dat;
    }

    base
}

/// Simulate on a single thread.
fn single_thread(_thread_id: usize, pb: &Arc<Mutex<ParBar>>, input: &Input) -> Histogram {
    let mut data = Histogram::new(input.min, input.max, input.bins);

    let mut rng = thread_rng();

    while let Some((start, end)) = {
        let mut pb = pb.lock().expect("Could not lock progress bar.");
        let b = pb.block(input.block_size);
        std::mem::drop(pb);
        b
    } {
        for _ in start..end {
            let x = input.dist.gen(&mut rng);

            data.try_collect(x);
        }
    }

    data
}

/// Save the output data.
fn save(out_dir: &Path, data: Histogram) {
    banner::section("Saving");
    let path = out_dir.join("histogram.csv");
    data.save(&path).expect("Failed to write output file.");
}
