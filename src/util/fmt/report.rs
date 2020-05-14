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

// /// Report a value and either its associated name, or a human readable string if supplied.
// #[macro_export]
// macro_rules! report {
//     ($var: expr) => {};

//     ($desc: tt, $var: expr) => {
//         println!("{:>32} :  {}", $desc, $var);
//     };

//     ($desc: tt, $var: expr, $units: tt) => {
//         println!("{:>32} :  {} [{}]", $desc, $var, $units);
//     };
// }

// /// Report a list of values and either an associated name, or a human readable string if supplied.
// #[macro_export]
// macro_rules! report_list {
//     ($var: expr) => {
//         println!("{:>32} :  {}", stringify!($var), dia::fmt::list($var));
//     };

//     ($desc: tt, $var: expr) => {
//         println!("{:>32$} :  {}", $desc, dia::fmt::list($var));
//     };

//     ($desc: tt, $var: expr, $units: tt) => {
//         println!("{:>32} :  {} [{}]", $desc, dia::fmt::list($var), $units);
//     };
// }
