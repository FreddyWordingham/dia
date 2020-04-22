//! Ray-tracing module.

pub mod ray;
pub mod side;
pub mod trace;

pub use self::{ray::*, side::*, trace::*};
