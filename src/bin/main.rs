//! Main library test binary.

use proc::HelloMacro;

#[derive(HelloMacro)]
struct Testy {
    /// Place holder item.
    item: u64,
}

/// Main function.
pub fn main() {
    println!("Hello world!");

    let t = Testy { item: 4 };
    Testy::hello();
    println!("The item is: {}", t.item);
}
