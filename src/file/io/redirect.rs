//! Redirection implementation.

use crate::{Error, Load};
// use attr::load;
use std::path::{Path, PathBuf};

/// Redirect the loading path.
#[derive(Debug, serde::Deserialize)]
pub enum Redirect<T: Load> {
    /// Given values.
    Here(T),
    /// File redirection.
    File(PathBuf),
}

impl<T: Load> Redirect<T> {
    /// Load the instance.
    /// # Errors
    /// if the redirection can not be opened
    /// or if the data can not be serialised into a valid instance.
    #[inline]
    pub fn load(self, in_dir: &Path) -> Result<T, Error> {
        match self {
            Self::Here(data) => Ok(data),
            Self::File(path) => {
                let path = in_dir.join(&path);
                T::load(&path)
            }
        }
    }
}
