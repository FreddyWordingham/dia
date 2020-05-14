use dia::report;
use rand::Rng;
use std::f64::consts::PI;

/// Number of samples to take.
const SAMPLES: u64 = 1_000_000;

/// Estimate the value of PI.
fn main() {
    // Create a new random number generator.
    let mut rng = rand::thread_rng();

    // Record the number of samples that fall within the circle's radius.
    let mut inside_circle = 0;

    // Do it SAMPLES times.
    for _ in 0..SAMPLES {
        // Random coordinates.
        let x = rng.gen_range(-1.0, 1.0);
        let y = rng.gen_range(-1.0, 1.0);

        // Use squared distance to save sqrt.
        let dist_sq = (x * x) + (y * y);
        if dist_sq <= 1.0 {
            inside_circle += 1;
        }
    }

    // Calculate estimated value and accuracy.
    let pi_estimate = 4.0 * (inside_circle as f64 / SAMPLES as f64);
    let accuracy = pi_estimate / PI;

    // Print values.
    report::obj("pi estimate", pi_estimate).unwrap();
    report::obj_units("delta", (accuracy - 1.0) * 100.0, "%").unwrap();
}
