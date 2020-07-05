//! Install directory information.

use crate::util::exec;
use std::{
    env::{current_dir, set_current_dir},
    fs::create_dir_all,
    path::PathBuf,
};

/// Initialise the current working directory.
#[inline]
fn input_dir(dir: &PathBuf) -> Result<PathBuf, std::io::Error> {
    set_current_dir(dir)?;
    current_dir()
}

/// Create an output directory.
#[inline]
fn output_dir(dir: &PathBuf) -> Result<PathBuf, std::io::Error> {
    create_dir_all(dir)?;
    Ok(dir.to_path_buf())
}

/// Set and get the input and output directories.
/// Returned pair is (input, output).
/// # Errors
/// if the root installation directory can not be determined,
/// or if one of the input or output directories could not be created.
#[inline]
pub fn io_dirs(
    input: Option<PathBuf>,
    output: Option<PathBuf>,
) -> Result<(PathBuf, PathBuf), crate::Error> {
    let exec_name = exec::name()?;

    let cwd = current_dir()?;

    let in_dir = if let Some(input) = input {
        input
    } else {
        cwd.join("input").join(&exec_name)
    };

    let out_dir = if let Some(output) = output {
        output
    } else {
        cwd.join("output").join(exec_name)
    };

    let in_dir = input_dir(&in_dir)?;
    let out_dir = output_dir(&out_dir)?;
    Ok((in_dir, out_dir))
}
