//! Rendering binary.

use attr::input;
use dia::*;
// use palette::{Gradient, LinSrgba};
// use rand::thread_rng;
use std::{env::current_dir, path::PathBuf};

/// Input parameters.
#[input]
struct Parameters {
    // /// Adaptive mesh settings.
// tree: tree::Settings,
// /// Regular grid settings.
// grid: grid::Settings,
// /// Render runtime settings.
// sett: render::Settings,
// /// Surfaces map.
// surfs: Set<form::Mesh>,
// /// Colour map.
// cols: Set<form::Gradient>,
// /// Attributes map.
// attrs: Set<Redirect<render::Attributes>>,
// /// Camera setup.
// cam: form::Camera,
}

/// Main function.
pub fn main() {
    banner::title("Render");
    let (_params_path, _in_dir, _out_dir) = init();
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
    let cwd = current_dir().expect("Failed to determine current working directory.");
    let exec_name = exec::name().expect("Could not determine executable name.");
    let (in_dir, out_dir) = dir::io_dirs(
        Some(cwd.join("input").join(exec_name.clone()§)),
        Some(cwd.join("output").join(exec_name)),
    )
    .expect("Could not initialise directories");
    report!("input directory", in_dir.display());
    report!("output directory", out_dir.display());

    (params_path, in_dir, out_dir)
}
