//! Rendering simulation functions.

use crate::{
    render::{Input, Output, Scene},
    ParBar,
};
/// Render an image.
/// # Errors
/// if an invalid thread image was created.
#[inline]
pub fn simulate(input: &Input, scene: &Scene) -> Output {
    // let num_pixels = input.cam.sensor().num_pixels();
    let width = scene.cam().sensor().res().0 as usize;
    let height = scene.cam().sensor().res().1 as usize;

    Output::new([width, height])
}
