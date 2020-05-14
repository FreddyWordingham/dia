//! Report macro.

use std::fmt::Display;

/// Report a value and either its associated name, or a human readable string if supplied.
#[macro_export]
macro_rules! report {
    ($var: expr) => {
        println!("{:>32} :  {}", stringify!($var), $var);
    };

    ($desc: tt, $var: expr) => {
        println!("{:>32} :  {}", $desc, $var);
    };

    ($desc: tt, $var: expr, $units: tt) => {
        println!("{:>32} :  {} [{}]", $desc, $var, $units);
    };
}
