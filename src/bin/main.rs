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

    dia::title("Title");
    dia::section("Section");
    dia::sub_section("Sub-Section");
    dia::sub_sub_section("Sub-Sub-Section");
    dia::section("Section two");
    dia::sub_section("Sub-Section two");
    dia::sub_sub_section("Sub-Sub-Section two");

    let x = 2;
    dia::report!(x);
}
