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

    dia::banner::title("Title");
    dia::banner::section("Section");
    dia::banner::sub_section("Sub-Section");
    dia::banner::sub_sub_section("Sub-Sub-Section");
    dia::banner::section("Section two");
    dia::banner::sub_section("Sub-Section two");
    dia::banner::sub_sub_section("Sub-Sub-Section two");

    dia::dir::io_dirs(None, None).expect("Could not initialise directories.");
}
