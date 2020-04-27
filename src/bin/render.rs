//! Main library test binary.

use attr::input;
use dia::*;
use std::path::{Path, PathBuf};

/// Input parameters.
#[input]
struct Parameters {
    /// Place holder item.
    item: u64,
}

/// Main function.
pub fn main() {
    banner::title("Render");
    let (in_dir, _out_dir, params_path) = init();
    let params = input(&in_dir, &params_path);
    let (_scene) = setup(&params);
}

fn init() -> (PathBuf, PathBuf, PathBuf) {
    banner::section("Initialisation");
    banner::sub_section("Directories");
    let (in_dir, out_dir) = dir::io_dirs(None, None).expect("Could not initialise directories.");
    report!("in_dir", in_dir.display());
    report!("out_dir", out_dir.display());

    banner::sub_section("Command line args");
    args!(bin_path: PathBuf;
        params_path: PathBuf
    );
    report!(bin_path);
    report!(params_path);

    (in_dir, out_dir, params_path)
}

fn input(in_dir: &Path, params_path: &Path) -> Parameters {
    banner::section("Input");
    banner::sub_section("Parameters");
    let params =
        Parameters::load(&in_dir.join(params_path)).expect("Could not load parameters file.");

    params
}

fn setup(params: &Parameters) -> Set<Vec<Mesh>> {
    
}
