//! Rendering binary.

use attr::input;
use dia::*;
use std::path::{Path, PathBuf};

/// Input parameters.
#[input]
struct Parameters {
    /// Adaptive mesh settings.
    tree: tree::Settings,
    /// Regular grid settings.
    grid: grid::Settings,
    /// Render runtime settings.
    sett: render::Settings,
    /// Surfaces map.
    surfs: Set<form::Mesh>,
    /// Colour map.
    cols: Set<form::Gradient>,
    /// Attributes map.
    attrs: Set<Redirect<render::Attributes>>,
    /// Camera setup.
    cam: form::Camera,
}

/// Main function.
pub fn main() {
    banner::title("Render");
    let (params_path, in_dir, _out_dir) = init();
    let params = input(&in_dir, &params_path);
    let (_tree_sett, _grid_sett) = build(&in_dir, params);

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

    let params = Parameters::load(&path).expect("Could not load parameters file");
    // report!("Input parameters", format!("{:#?}", params));

    params
}

/// Build instances.
fn build(_in_dir: &Path, params: Parameters) -> (tree::Settings, grid::Settings) {
    banner::section("Building");
    banner::sub_section("Adaptive Tree Settings");
    let tree_sett = params.tree;
    report!("Tree settings", &tree_sett);

    banner::sub_section("Grid Settings");
    let grid_sett = params.grid;
    report!("Grid settings", &grid_sett);

    (tree_sett, grid_sett)
}
