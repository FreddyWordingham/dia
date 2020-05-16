//! MCRT binary.

use attr::input;
use dia::*;
use std::path::{Path, PathBuf};

/// Input parameters.
#[input]
struct Parameters {
    /// Adaptive mesh settings.
    tree: tree::Settings,
    /// MCRT runtime settings.
    sett: mcrt::Settings,
    /// Light settings.
    light: form::Light,
    /// Surfaces map.
    surfs: Set<form::Mesh>,
    /// Properties map.
    props: Set<Redirect<form::Properties>>,
}

/// Main function.
pub fn main() {
    banner::title("MCRT");
    let (params_path, in_dir, _out_dir) = init();
    let params = input(&in_dir, &params_path);
    build(&in_dir, params);
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

    banner::sub_sub_section("Grid");
    report!("tree", &params.tree);

    banner::sub_sub_section("Settings");
    report!("settings", &params.sett);

    banner::sub_sub_section("Light");
    report!("light", &params.light);

    banner::sub_sub_section("Surfaces");
    report!("surfaces", &params.surfs);

    banner::sub_sub_section("Properties");
    report!("properties", &params.props);

    params
}

/// Build instances.
fn build(in_dir: &Path, params: Parameters) {
    banner::section("Building");

    banner::sub_section("Light");
    let light = params.light.build(in_dir).expect("Unable to build light.");
    report!(light);

    banner::sub_section("Surfaces");
    let surfs = params
        .surfs
        .build(in_dir)
        .expect("Unable to build surfaces.");
    report!(&surfs);

    banner::sub_section("Properties");
    let props = params
        .props
        .build(in_dir)
        .expect("Unable to build properties.");
    report!(props);

    banner::sub_section("Tree");
    let _tree = tree::Cell::new_root(&params.tree, &surfs);
}
