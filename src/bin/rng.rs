//! Formula testing binary.

use attr::input;
use dia::*;
use std::path::{Path, PathBuf};

/// Input parameters.
#[input]
struct Parameters {
    /// Number of sample to take of the distribution.
    samples: u64,
}

/// Main function.
pub fn main() {
    banner::title("RNG Testing");
    let (params_path, in_dir, out_dir) = init();
    let params = input(&in_dir, &params_path);
    let samples = build(&in_dir, params);
    let data = simulate(samples);
    save(&out_dir, data);
    banner::section("Finished");
}

/// Initialise the command line arguments and directories.
fn init() -> (PathBuf, PathBuf, PathBuf) {
    banner::section("Initialisation");
    banner::sub_section("Command line args");
    args!(bin_path: PathBuf;
        params_path: PathBuf
    );
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
    let params = Parameters::load(&path).expect("Could not load parameters file");

    params
}

/// Build instances.
fn build(_in_dir: &Path, params: Parameters) -> u64 {
    banner::section("Building");

    params.samples
}

/// Run the simulation.
fn simulate(samples: u64) -> Histogram {
    banner::section("Simulating");

    for _ in 0..samples {
        //     let x = start + (delta * n as f64);
        //     let y = formula.y(x);
        //     data.push((x, y));
    }

    let min = 0.0;
    let max = 1.0;
    let bins = 100;
    let data = Histogram::new(min, max, bins);
    data
}

/// Save the output data.
fn save(out_dir: &Path, data: Histogram) {
    banner::section("Saving");
    let path = out_dir.join("histogram.csv");
    data.save(&path).expect("Failed to write output file.");
}
