//! Load trait.

use crate::file::Error;
use json5;
use serde::Deserialize;
use std::{fs::read_to_string, path::Path};

/// Types implementing this trait can be loaded from a file.
pub trait Load {
    /// Deserialize the type from a given file.
    fn load(path: &Path) -> Self;
}

/// Deserialise the type in json format.
/// # Errors
/// if file can not be opened or read string isn't a valid json instance.
#[inline]
pub fn from_json<T>(path: &Path) -> Result<T, Error>
where
    for<'de> T: Deserialize<'de>,
{
    let s = read_to_string(path)?;
    Ok(json5::from_str(&s)?)
}
