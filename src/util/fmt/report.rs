//! Report macro.

/// Report a value and either its associated name, or a human readable string if supplied.
#[macro_export]
macro_rules! report {
    ($var: expr) => {
        println!("{:>16}  : {:?}", stringify!($var), $var);
    };

    ($desc: tt, $var: expr) => {
        println!("{:>16}  : {}", $desc, $var);
    };

    ($desc: tt, $var: expr, $units: tt) => {
        println!("{:>16}  : {} [{}]", $desc, $var, $units);
    };
}
