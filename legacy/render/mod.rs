//! Rendering module.

pub mod attribute;
pub mod camera;
pub mod illumination;
pub mod paint;
pub mod run;
pub mod scene;
pub mod settings;

pub use self::{attribute::*, camera::*, illumination::*, scene::*, settings::*};
