//! Input and output module.

pub mod load;
pub mod redirect;
pub mod save;

pub use self::{load::*, redirect::*, save::*};
