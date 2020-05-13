//! MCRT binary.

use attr::input;
use dia::*;
use std::path::{Path, PathBuf};

/// Input parameters.
#[input]
struct Parameters {
    /// Adaptive mesh settings.
    amr: settings::Adaptive,
    /// MCRT runtime settings.
    sett: mcrt::Settings,
    /// Light settings.
    light: form::Light,
    /// Input surfaces.
    surfs: Vec<(Group, Vec<String>)>,
    /// Physical attributes.
    props: Vec<(Group, Redirect<form::Properties>)>,
}

/// Main function.
pub fn main() {
    banner::title("MCRT");
    let (params_path, in_dir, _out_dir) = init();
    let params = input(&in_dir, &params_path);
    let grid = setup(&in_dir, params);
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
    let params =
        Parameters::load(&in_dir.join(params_path)).expect("Could not load parameters file");

    banner::sub_sub_section("Grid");
    report!("grid", format!("_\n{}", params.amr));

    banner::sub_sub_section("Settings");
    report!("mcrt", format!("_\n{}", params.sett));

    banner::sub_sub_section("Light");
    report!("light", format!("_\n{}", params.light));

    banner::sub_sub_section("Surfaces");
    report!(
        "surfaces",
        format!(
            "_\n{}",
            slice::groups_list(params.surfs.as_slice()).expect("Could not format list.")
        )
    );

    banner::sub_sub_section("Properties");
    report!(
        "properties",
        format!(
            "_\n{}",
            slice::groups(params.props.as_slice()).expect("Could not format list.")
        )
    );

    params
}

/// Load second-dependency files.
fn setup(in_dir: &Path, params: Parameters) {
    banner::section("Setup");
    banner::sub_section("Parameters");
    build_from_list(in_dir, params.props);
}
