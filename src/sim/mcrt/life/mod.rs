//! Photon lifetime function module.

use crate::mcrt::{Input, Output};
use rand::rngs::ThreadRng;

/// Photon lifetime function type.
pub type Life = fn(&Input, &mut Output, &mut ThreadRng);

pub mod test;

pub use self::test::*;
