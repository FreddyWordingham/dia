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
        write!(s, "{:>15} ", v)?;
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
pub fn groups<T, S>(groups: &[(T, S)]) -> Result<String, Error>
where
    T: Display,
    S: Display,
{
    let mut s = String::new();
    for (group, val) in groups {
        writeln!(s, "{:>16} >  {}", format!("[{}]", group), val)?;
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
pub fn groups_list<T, S>(groups: &[(T, Vec<S>)]) -> Result<String, Error>
where
    T: Display,
    S: Display,
{
    let mut s = String::new();
    for (group, vals) in groups {
        let vs = values(vals.as_slice())?;
        writeln!(s, "{:>16} >  {}", format!("[{}]", group), vs)?;
    }

    if !s.is_empty() {
        s.pop();
    }

    Ok(s)
}
