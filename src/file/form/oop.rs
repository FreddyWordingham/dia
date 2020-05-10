//! File re-direction implementation.

use crate::{Error, Load};
use std::path::Path;

/// Possible file redirection structure.
#[derive(Debug, serde::Deserialize)]
pub enum Oop<T: Load> {
    /// Path to file.
    There(String),
    /// Direct value.
    Here(T),
}

impl<T: Clone + Load> Oop<T> {
    /// Access the held value, or load it from the file.
    /// # Errors
    /// if the file can not be loaded.
    #[inline]
    pub fn get(&self, in_dir: &Path) -> Result<T, Error> {
        match self {
            Self::There(path) => T::load(&in_dir.join(path)),
            Self::Here(val) => Ok((*val).clone()),
        }
    }
}

impl<T: Load> Load for Oop<T>
where
    for<'de> T: serde::Deserialize<'de>,
{
    #[inline]
    fn load(path: &std::path::Path) -> std::result::Result<Self, crate::Error> {
        crate::from_json(path)
    }
}
