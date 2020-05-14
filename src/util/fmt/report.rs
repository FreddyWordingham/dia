//! Report macro.

use std::fmt::Display;
use std::fmt::Write;

/// Length allocated to name printing.
const NAME_LENGTH: usize = 32;

/// Report the value of an object.
#[inline]
pub fn obj<T: Display>(name: &str, obj: T) {
    let mut s = String::new();
    write!(s, "{}", obj).expect("Unable to format object.");
    if s.contains('\n') {
        println!("{:>name_len$} :  _\n{}", name, s, name_len = NAME_LENGTH);
    } else {
        println!("{:>name_len$} :  {}", name, s, name_len = NAME_LENGTH);
    }
}

/// Report the value of an object.
#[inline]
pub fn obj_units<T: Display>(name: &str, obj: T, units: &str) {
    let mut s = String::new();
    write!(s, "{}", obj).expect("Unable to format object.");
    if s.contains('\n') {
        println!(
            "{:>name_len$} :  _\n{} [{}]",
            name,
            s,
            units,
            name_len = NAME_LENGTH
        );
    } else {
        println!(
            "{:>name_len$} :  {} [{}]",
            name,
            s,
            units,
            name_len = NAME_LENGTH
        );
    }
}

/// Report the a list of items.
#[inline]
pub fn list<T: Display>(name: &str, list: &[T]) {
    let mut s = String::new();
    for item in list {
        write!(s, "{:>15} ", item).expect("Unable to format item.");
    }

    if !s.is_empty() {
        s.pop();
    }

    obj(name, s)
}
