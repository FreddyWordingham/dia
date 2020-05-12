//! Executable information.

use crate::Error;
use std::{env::args, path::Path};

/// Determine the name of the executable.
#[inline]
#[must_use]
pub fn name() -> Result<String, Error> {
    let args: Vec<String> = args().collect();
    let name = &args[0];

    Ok(Path::new(name)
        .file_name()
        .ok_or("Missing filename.")?
        .to_str()
        .ok_or("Missing string.")?
        .to_string())
}
