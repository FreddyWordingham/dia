//! Lighting setup module.

pub mod light;
pub mod samples;
pub mod shadow;
pub mod sky;

pub use self::{light::*, samples::*, shadow::*, sky::*};

//  /// Conglomerate lighting setup structure.
