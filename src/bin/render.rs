//! Rendering binary.

use attr::input;
use dia::*;
use palette::{Gradient, LinSrgba};
use rand::thread_rng;
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
    let (params_path, in_dir, out_dir) = init();
    let params = input(&in_dir, &params_path);
    let (tree_sett, grid_sett, render_sett, surfs, cols, attrs, cam, perl) = build(&in_dir, params);
    let (tree, grid) = grow(tree_sett, grid_sett, &surfs);
    let input = render::Input::new(
        &render_sett,
        &surfs,
        &cols,
        &attrs,
        &cam,
        &tree,
        &grid,
        &perl,
    );
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
    render::Settings,
    Set<Mesh>,
    Set<Gradient<LinSrgba>>,
    Set<render::Attributes>,
    render::Camera,
    PerlinMap,
) {
    banner::section("Building");
    banner::sub_section("Adaptive Tree Settings");
    let tree_sett = params.tree;
    report!("Tree settings", &tree_sett);

    banner::sub_section("Grid Settings");
    let grid_sett = params.grid;
    report!("Grid settings", &grid_sett);

    banner::sub_section("Render Settings");
    let render_sett = params.sett;
    report!("Render settings", &render_sett);

    banner::sub_section("Surfaces");
    let surfs = params
        .surfs
        .build(in_dir)
        .expect("Unable to build surfaces.");
    report!("Surfaces", &surfs);

    banner::sub_section("Colours");
    let cols = params
        .cols
        .build(in_dir)
        .expect("Unable to build gradients.");
    for (group, grad) in cols.map() {
        report!(&format!("[{}]", group), gradient::to_string(&grad, 32));
    }

    banner::sub_section("Attributes");
    let attrs = params
        .attrs
        .build(in_dir)
        .expect("Unable to build attributes.");
    report!("Attributes", &attrs);

    banner::sub_section("Camera");
    let cam = params.cam.build(in_dir).expect("Unable to build camera.");
    report!("Camera", &cam);

    banner::sub_section("Noise Map");
    let perl = PerlinMap::new(render_sett.perl_segs(), &mut thread_rng());
    report!("Perlin map", &perl);

    (
        tree_sett,
        grid_sett,
        render_sett,
        surfs,
        cols,
        attrs,
        cam,
        perl,
    )
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

/// Run the renderer.
fn render(input: &render::Input) -> render::Output {
    banner::section("Rendering");
    banner::sub_section("Main Camera");
    render::run::simulate(&input, render::naboo)
}

/// Save the output data.
fn save(out_dir: &Path, data: render::Output) {
    banner::section("Saving");
    banner::sub_section("Main Dump");
    data.save(&out_dir).expect("Could not save output data.");
}
