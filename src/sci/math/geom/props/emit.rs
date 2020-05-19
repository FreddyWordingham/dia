//! Emit trait.

use rand::rngs::ThreadRng;

/// Emit trait implementation.
/// Types implementing this trait can generate Rays.
pub trait Emit {
    /// Check for an overlapping collision.
    fn emit(&self, rng: &mut ThreadRng) -> bool;
}
