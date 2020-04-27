//! Main library test binary.

use attr::input;

#[input]
struct Testy {
    /// Place holder item.
    item: u64,
}

/// Main function.
pub fn main() {
    println!("Hello world!");
}
