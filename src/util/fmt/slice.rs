//! Iterator formatting function.

use crate::Error;
use std::fmt::{Display, Write};

/// Print the vector of values to a string.
/// # Errors
/// if can not write value to string.
#[inline]
pub fn values<T>(vals: &[T]) -> Result<String, Error>
where
    T: Display,
{
    let mut s = String::new();
    for v in vals {
        write!(s, "{:>28} ", v)?;
    }

    if !s.is_empty() {
        s.pop();
    }

    Ok(s)
}

/// Print the group and vector of values to a string.
/// # Errors
/// if can not write values to string.
#[inline]
pub fn groups<T, S>(groups: &[(T, Vec<S>)]) -> Result<String, Error>
where
    T: Display,
    S: Display,
{
    let mut s = String::new();
    for (group, vals) in groups {
        let vs = values(vals.as_slice())?;
        write!(s, "\n{:>32}  : {}", group, vs)?;
    }

    Ok(s)
}
