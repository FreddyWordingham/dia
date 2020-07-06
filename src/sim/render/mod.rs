//! Rendering module.

pub mod attributes;
pub mod camera;
pub mod event;
pub mod illumination;
pub mod input;
pub mod lighting;
pub mod output;
pub mod painter;
pub mod run;
pub mod scene;
pub mod settings;

pub use self::{
    attributes::*, camera::*, event::*, illumination::*, input::*, lighting::*, output::*,
    painter::*, run::*, scene::*, settings::*,
};
