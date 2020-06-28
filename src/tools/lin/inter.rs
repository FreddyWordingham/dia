//! Interpolation functions.

/// Smooth-step function.
#[inline]
#[must_use]
pub fn smooth_step(a: f64, b: f64, x: f64) -> f64 {
    debug_assert!(x >= 0.0);
    debug_assert!(x <= 1.0);

    let y = (x - a) / (b - a);
    y.powi(2) * (3.0 - (2.0 * y))
}
