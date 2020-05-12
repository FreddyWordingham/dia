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
    attrs: Vec<(Group, Vec<String>)>,
}

/// Main function.
pub fn main() {
    banner::title("MCRT");
    let (params_path, in_dir, _out_dir) = init();
    let _params = input(&in_dir, &params_path);
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
    report!("tar_tris", params.amr.tar_tris());
    report!("max_depth", params.amr.max_depth());
    report!("padding", params.amr.padding() * 100.0, "%");

    banner::sub_sub_section("MCRT Settings");
    report!("bump_dist", params.sett.bump_dist() / 1.0e-9, "nm");

    banner::sub_sub_section("Surfaces");
    report!(
        "surfaces",
        format!("\n{}", slice::groups(&params.surfs).expect("Print failed."))
    );

    // let x = params.light.spec().get(&in_dir).expect("Sub load failed.");
    // let mut rng = rand::thread_rng();
    // report!("x", x.sample(&mut rng));

    params
}
