//! Sample binary.

use attr::input;
use dia::*;
use std::path::{Path, PathBuf};

/// Input parameters.
#[input]
struct Parameters {
    /// Linear gradient.
    m: f64,
    /// Linear offset.
    c: f64,
}

/// Main function.
pub fn main() {
    banner::title("Sample");
    let (params_path, in_dir, _out_dir) = init();
    let params = input(&in_dir, &params_path);
    let _gen = build(params);
    banner::section("Finished");
}

/// Initialise the command line arguments and directories.
fn init() -> (PathBuf, PathBuf, PathBuf) {
    banner::section("Initialisation");
    banner::sub_section("Command line arguments");
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

    Parameters::load(&path).expect("Could not load parameters file")
}

/// Build instances.
fn build(params: Parameters) {
    banner::section("Building");
    banner::sub_section("Linear Generator");

    let _m = params.m;
    let _c = params.c;
    let gen = ();
    // report!("Generator", &gen);

    gen
}
