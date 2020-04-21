//! Executable information.

use std::{env::args, path::Path};

/// Determine the name of the executable.
#[inline]
#[must_use]
pub fn name() -> String {
    let args: Vec<String> = args().collect();
    let name = args.get(0).unwrap();

    Path::new(&name)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}
