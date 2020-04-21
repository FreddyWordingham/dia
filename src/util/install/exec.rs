//! Executable information.

use std::{env::args, path::Path};

/// Determine the name of the executable.
#[inline]
#[must_use]
pub fn name() -> Option<String> {
    let args: Vec<String> = args().collect();
    let name = args.get(0)?;

    Some(Path::new(&name).file_name()?.to_str()?.to_string())
}
