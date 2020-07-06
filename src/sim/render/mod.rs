//! Rendering module.

pub mod attributes;
pub mod camera;
pub mod scene;
// pub mod event;
// pub mod illumination;
// pub mod input;
// pub mod output;
// pub mod painter;
// pub mod run;
pub mod lighting;
pub mod settings;

// pub use self::{attributes::*, camera::*, event::*, input::*, output::*, painter::*, settings::*};
pub use self::{attributes::*, camera::*, lighting::*, scene::*, settings::*};
