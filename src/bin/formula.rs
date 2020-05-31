//! Formula testing binary.

use attr::input;
use dia::*;
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

/// Input parameters.
#[input]
struct Parameters {
    /// Formula.
    formula: form::Formula,
    /// Starting value.
    start: f64,
    /// End value.
    end: f64,
    /// Number of sample to take of the formula.
    samples: u64,
}

/// Main function.
pub fn main() {
    banner::title("Formula Testing");
    let (params_path, in_dir, out_dir) = init();
    let params = input(&in_dir, &params_path);
    let (formula, start, end, samples) = build(&in_dir, params);
    let data = simulate(&formula, start, end, samples);
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
fn build(in_dir: &Path, params: Parameters) -> (Formula, f64, f64, u64) {
    banner::section("Building");

    (
        params
            .formula
            .build(in_dir)
            .expect("Could not build formula."),
        params.start,
        params.end,
        params.samples,
    )
}

/// Run the simulation.
fn simulate(formula: &Formula, start: f64, end: f64, samples: u64) -> Vec<(f64, f64)> {
    banner::section("Simulating");

    let delta = (end - start) / (samples - 1) as f64;
    let mut data = Vec::with_capacity(samples as usize);
    for n in 0..samples {
        let x = start + (delta * n as f64);
        let y = formula.y(x);
        data.push((x, y));
    }

    data
}

/// Save the output data.
fn save(out_dir: &Path, data: Vec<(f64, f64)>) {
    banner::section("Saving");
    let path = out_dir.join("points.csv");
    let mut file = File::create(path).expect("Could not create output file.");

    for (x, y) in data {
        writeln!(file, "{:>32}, {:<32}", x, y).expect("Could not write to output file.");
    }
}
