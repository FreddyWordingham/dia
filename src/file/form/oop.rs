//! File re-direction implementation.

use crate::{Error, Load};
use std::path::Path;

/// Possible file redirection structure.
#[derive(Debug, serde::Deserialize)]
pub enum Oop<T> {
    /// Path to file.
    File(String),
    /// Direct value.
    Here(T),
}

impl<T: Load> Oop<T> {
    /// Access the held value, or load it from the file.
    #[inline]
    #[must_use]
    pub fn get(self, in_dir: &Path) -> Result<T, Error> {
        match self {
            Self::File(path) => T::load(&in_dir.join(path)),
            Self::Here(val) => Ok(val),
        }
    }
}

impl<T> Load for Oop<T>
where
    for<'de> T: serde::Deserialize<'de>,
{
    #[inline]
    fn load(path: &std::path::Path) -> std::result::Result<Self, crate::Error> {
        crate::from_json(path)
    }
}
