//! Formula testing binary.

use attr::input;
use dia::*;
use std::path::{Path, PathBuf};

/// Input parameters.
#[input]
struct Parameters {}

/// Main function.
pub fn main() {
    banner::title("Formula Testing");
    let (params_path, in_dir, _out_dir) = init();
    let params = input(&in_dir, &params_path);
    let data = simulate(&params);
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

/// Run the simulation.
fn simulate(_params: &Parameters) {
    banner::section("Simulating");

    let formula = Formula::Polynomial::
}
