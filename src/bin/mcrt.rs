//! MCRT binary.

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
    let (params_path, in_dir, out_dir) = init();
    let params = input(&in_dir, &params_path);
    let (tree_sett, grid_sett, mcrt_sett, light, surfs, props) = build(&in_dir, params);
    let (tree, grid) = grow(tree_sett, grid_sett, &surfs);
    let input = mcrt::Input::new(&mcrt_sett, &light, &props, &tree, &grid);
    let data = render(&input);
    save(&out_dir, data);
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
#[allow(clippy::type_complexity)]
fn build(
    in_dir: &Path,
    params: Parameters,
) -> (
    tree::Settings,
    grid::Settings,
    mcrt::Settings,
    mcrt::Light,
    Set<Mesh>,
    Set<mcrt::Properties>,
) {
    banner::section("Building");
    banner::sub_section("Adaptive Tree Settings");
    let tree_sett = params.tree;
    report!("Tree settings", &tree_sett);

    banner::sub_section("Grid Settings");
    let grid_sett = params.grid;
    report!("Grid settings", &grid_sett);

    banner::sub_section("MCRT Settings");
    let mcrt_sett = params.sett;
    report!("MCRT settings", &mcrt_sett);

    banner::sub_section("Light");
    let light = params.light.build(in_dir).expect("Unable to build light.");
    report!("Light", &light);

    banner::sub_section("Surfaces");
    let surfs = params
        .surfs
        .build(in_dir)
        .expect("Unable to build surfaces.");
    report!("Surfaces", &surfs);

    banner::sub_section("Properties");
    let props = params
        .props
        .build(in_dir)
        .expect("Unable to build properties.")
        .build(in_dir)
        .expect("Unable to build properties.");
    report!("Properties", &props);

    (tree_sett, grid_sett, mcrt_sett, light, surfs, props)
}

/// Grow domains.
fn grow<'a>(
    tree_sett: tree::Settings,
    grid_sett: grid::Settings,
    surfs: &'a Set<Mesh>,
) -> (tree::Cell<'a>, grid::Grid) {
    banner::section("Growing");

    banner::sub_section("Adaptive Tree");
    let tree = tree::Cell::new_root(&tree_sett, &surfs);
    report!("Adaptive tree", &tree);

    banner::sub_section("Regular Grid");
    let grid = grid::Grid::new(&grid_sett);
    report!("Regular grid", &grid);

    (tree, grid)
}

/// Run the mcrt sim.
fn render(input: &mcrt::Input) -> mcrt::Output {
    banner::section("Simulating");
    banner::sub_section("Main Light");
    mcrt::run::simulate(&input, mcrt::life::test).expect("Simulation failed.")
}

/// Save the output data.
fn save(out_dir: &Path, data: mcrt::Output) {
    banner::section("Saving");
    banner::sub_section("Main Dump");
    data.save(&out_dir).expect("Could not save output data.");
}
