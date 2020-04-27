//! Frame input settings implementation.

use crate::{access, clone, AspectRatio, Pos3};
use attr::load;

/// Frame settings.
#[load]
pub struct Frame {
    /// Aspect ratio.
    aspect_ratio: AspectRatio,
    /// Horizontal resolution.
    hr_res: f64,
    /// Horizontal field of view.
    fov: f64,
    /// Position of the camera.
    cam_pos: Pos3,
    /// Target of the camera.
    tar_pos: Pos3,
}

impl Frame {
    clone!(aspect_ratio, AspectRatio);
    clone!(fov, f64);
    clone!(hr_res, f64);
    access!(cam_pos, Pos3);
    access!(tar_pos, Pos3);
}
