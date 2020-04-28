//! Main library test binary.

use attr::input;
use dia::*;
use std::path::{Path, PathBuf};

/// Scene parameters.
#[input]
struct SceneParameters {
    /// Input surfaces.
    surfs: Vec<(Group, Vec<String>)>,
}

/// Input parameters.
#[input]
struct Parameters {
    /// Grid settings.
    grid: settings::Adaptive,
    /// Scene.
    scene: SceneParameters,
    /// Camera.
    cam: form::Camera,
    /// Colours.
    cols: form::Colours,
}

/// Main function.
pub fn main() {
    banner::title("Render");
    let (params_path, in_dir, out_dir) = init();
    let params = input(&in_dir, &params_path);
    let scene = setup(&in_dir, &params);
    let (grid, cam) = build(&params, &scene);
    let img = render(&grid, &cam);
    save(&out_dir, img);
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
    report!("max depth", params.grid.max_depth());
    report!("tar tris", params.grid.tar_tris());
    report!("padding factor", params.grid.padding());

    params
}

/// Load the resource files.
fn setup(in_dir: &Path, params: &Parameters) -> Scene {
    banner::section("Input");
    banner::sub_section("Scene");
    let mut names: Set<Vec<String>> = Set::new();
    for (group, meshes) in &params.scene.surfs {
        for mesh in meshes {
            if let Some(entry) = names.get_mut(group) {
                entry.push(mesh.clone());
            } else {
                names.insert(*group, vec![mesh.clone()]);
            }
        }
    }
    let scene = Scene::load(&in_dir.join("meshes"), names.iter()).expect("Could not load scene.");

    report!("num groups", scene.surfs().len());

    scene
}

/// Build the simulation structures.
fn build<'a>(params: &Parameters, scene: &'a Scene) -> (Adaptive<'a>, render::Camera) {
    banner::section("Building");
    banner::sub_section("Adaptive grid");
    let grid = Adaptive::new_root(&params.grid, scene);

    report!("max depth", grid.max_depth());
    report!("num cells", grid.num_cells());
    report!("num leaf cells", grid.num_leaf_cells());
    report!("num tri refs", grid.num_tri_refs());
    report!("ave leaf tris", format!("{:.2}", grid.ave_leaf_tris()));

    banner::sub_section("Camera");
    let cam = params.cam.build();

    report!("position", cam.focus().orient().pos());
    report!("target", cam.focus().tar());
    report!("field of view", cam.lens().fov().to_degrees(), "deg");
    report!(
        "resolution",
        format!("{}x{}", cam.sensor().res().0, cam.sensor().res().1)
    );
    report!("total pixels", cam.sensor().num_pixels());

    (grid, cam)
}

/// Render an image.
fn render(grid: &Adaptive, cam: &render::Camera) -> Image {
    banner::section("Rendering");
    let img = render::run(&grid, &cam).expect("Rendering failed.");

    img
}

/// Save the image.
fn save(out_dir: &Path, img: Image) {
    banner::section("Saving");
    let path = out_dir.join("img.png");
    img.save(&path).expect("Failed to save image.");
    report!("Image saved at", path.display());
}
