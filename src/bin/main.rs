//! Main library test binary.

use attr::input;
use dia::*;
use std::path::PathBuf;

/// Input parameters.
#[input]
struct Parameters {
    /// Place holder item.
    item: u64,
}

/// Main function.
pub fn main() {
    banner::title("Render");
    let (in_dir, out_dir, params) = init();
    let input = input();
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

fn input() -> () {
    banner::section("Input");
}
