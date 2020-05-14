//! Iterator formatting function.

use crate::Error;
use std::fmt::{Display, Write};

/// Print a list of items to a string.
/// # Errors
/// if can not write item to string.
#[inline]
pub fn list<T>(list: &[T]) -> Result<String, Error>
where
    T: Display,
{
    let mut s = String::new();
    for item in list {
        write!(s, "{:>15} ", item)?;
    }

    if !s.is_empty() {
        s.pop();
    }

    Ok(s)
}
