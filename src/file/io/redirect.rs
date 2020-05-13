//! File re-direction implementation.

use crate::{Build, Error, Load};
use std::{
    fmt::{Display, Formatter},
    path::Path,
};

/// Possible file redirection structure.
#[derive(Debug, serde::Deserialize)]
pub enum Redirect<T> {
    /// Path to file.
    There(String),
    /// Direct value.
    Here(T),
}

impl<T: Load> Load for Redirect<T>
where
    for<'de> T: serde::Deserialize<'de>,
{
    #[inline]
    fn load(path: &std::path::Path) -> std::result::Result<Self, crate::Error> {
        crate::from_json(path)
    }
}

impl<T: Load> Build for Redirect<T> {
    type Inst = T;

    #[inline]
    fn build(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        match self {
            Self::There(path) => {
                let path = in_dir.join(path);
                println!("loading: {}", path.display());
                T::load(&path)
            }
            Self::Here(val) => Ok(val),
        }
    }
}

impl<T: Display> Display for Redirect<T> {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::There(path) => write!(fmt, "redirect -> {}", path),
            Self::Here(item) => write!(fmt, "{}", item),
        }
    }
}
