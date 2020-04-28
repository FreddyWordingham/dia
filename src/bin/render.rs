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
}

/// Main function.
pub fn main() {
    banner::title("Render");
    let (in_dir, _out_dir, params_path) = init();
    let params = input(&in_dir, &params_path);
    let scene = setup(&in_dir, &params);
    let (_grid, _cam) = building(&params, &scene);
}

fn init() -> (PathBuf, PathBuf, PathBuf) {
    banner::section("Initialisation");
    banner::sub_section("Directories");
    let (in_dir, out_dir) = dir::io_dirs(None, None).expect("Could not initialise directories");
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

fn building<'a>(params: &Parameters, scene: &'a Scene) -> (Adaptive<'a>, Camera) {
    banner::section("Building");
    banner::sub_section("Adaptive grid");
    let grid = Adaptive::new_root(&params.grid, scene);

    report!("max depth", grid.max_depth());
    report!("num cells", grid.num_cells());
    report!("num leaf cells", grid.num_leaf_cells());
    report!("num tri refs", grid.num_tri_refs());
    report!("ave leaf tris", grid.ave_leaf_tris());

    banner::sub_section("Camera");
    let cam = params.cam.build();

    report!("cam pos", cam.focus().orient().pos());
    report!("tar pos", cam.focus().tar());
    report!("field of view", cam.lens().fov(), "rad");
    report!(
        "resolution",
        format!("{}x{}", cam.sensor().res().0, cam.sensor().res().1)
    );
    report!("total pixels", cam.sensor().num_pixels());

    (grid, cam)
}
