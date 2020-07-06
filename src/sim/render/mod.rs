//! Rendering module.

pub mod attributes;
pub mod camera;
pub mod event;
pub mod scene;
// pub mod illumination;
pub mod input;
pub mod lighting;
pub mod output;
pub mod painter;
pub mod run;
pub mod settings;

pub use self::{
    attributes::*, camera::*, event::*, input::*, lighting::*, output::*, painter::*, run::*,
    scene::*, settings::*,
};
