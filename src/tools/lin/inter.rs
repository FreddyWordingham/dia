//! Interpolation functions.

/// Smooth-step function.
pub fn smooth_step(a: f64, b: f64, x: f64) -> f64 {
    debug_assert!(x >= 0.0);
    debug_assert!(x <= 1.0);

    let x = (x - a) / (b - a);
    x.powi(2) * (3.0 - (2.0 * x))
}
