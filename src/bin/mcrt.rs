//! MCRT binary.

use attr::input;
use dia::*;
use std::path::{Path, PathBuf};

/// Input parameters.
#[input]
struct Parameters {
    /// Adaptive mesh settings.
    tree: settings::Adaptive,
    /// MCRT runtime settings.
    sett: mcrt::Settings,
    /// Light settings.ccc
    light: form::Light,
    // /// Input surfaces.
    // surfs: Set<Vec<String>>,
    /// Physical attributes.
    props: Set<Redirect<form::Properties>>,
}

/// Main function.
pub fn main() {
    banner::title("MCRT");
    let (params_path, in_dir, _out_dir) = init();
    let params = input(&in_dir, &params_path);
    banner::section("Finished");
}

/// Initialise the command line arguments and directories.
fn init() -> (PathBuf, PathBuf, PathBuf) {
    banner::section("Initialisation");
    banner::sub_section("Command line args");
    args!(bin_path: PathBuf;
        params_path: PathBuf
    );
    report!(bin_path);
    report!(params_path);

    banner::sub_section("Directories");
    let (in_dir, out_dir) = dir::io_dirs(None, None).expect("Could not initialise directories");
    report!("in_dir", in_dir.display());
    report!("out_dir", out_dir.display());

    (params_path, in_dir, out_dir)
}

/// Load the input files.
fn input(in_dir: &Path, params_path: &Path) -> Parameters {
    banner::section("Input");
    banner::sub_section("Parameters");
    let path = in_dir.join(params_path);
    println!("loading: {}", path.display());
    let params = Parameters::load(&path).expect("Could not load parameters file");

    banner::sub_sub_section("Grid");
    report!("tree", format!("_\n{}", params.tree));

    banner::sub_sub_section("Settings");
    report!("mcrt", format!("_\n{}", params.sett));

    banner::sub_sub_section("Light");
    report!("light", format!("_\n{}", params.light));

    banner::sub_sub_section("Properties");
    report!("properties", format!("_\n{}", params.props));

    params
}

// /// Build instances.
// fn build(in_dir: &Path, params: Parameters) -> mcrt::Light {
//     banner::section("Building");
//     banner::sub_section("Light");
//     let light = params.light.build(in_dir).expect("Unable to build light.");
//     let _props = params
//         .props
//         .build(in_dir)
//         .expect("Unable to properties set.");

//     light
// }
