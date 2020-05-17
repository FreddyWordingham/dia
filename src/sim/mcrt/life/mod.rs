//! Photon lifetime function module.

use crate::mcrt::{Data, Input};
use rand::rngs::ThreadRng;

/// Photon lifetime function type.
pub type Life = fn(&Input, &mut Data, &mut ThreadRng);

pub mod test;

pub use self::test::*;
