//! Install directory information.

use std::{
    env::var,
    path::{Path, PathBuf},
};

/// Get the arc installation directory path from the environment variable.
/// Environment variable must be set.
/// # Errors
/// if the environment variable DIA_DIR is not set.
#[inline]
pub fn root() -> Result<PathBuf, std::env::VarError> {
    Ok(Path::new(&var("DIA_DIR")?).to_path_buf())
}
