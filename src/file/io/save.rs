//! Save trait.

use crate::{Error, X, Y};
use ndarray::Array2;
use netcdf::variable::Numeric;
use serde::Serialize;
use serde_json::to_string;
use std::fmt::Debug;
use std::{fs::write, path::Path};

/// Types implementing this trait can be saved to file.
pub trait Save {
    /// Serialise the type to a given file
    /// # Errors
    /// if the instance can not be serialised or if the file can't be written to.
    fn save(&self, path: &Path) -> Result<(), Error>;
}

/// Serialise the type in json format.
/// # Errors
/// if the instance can not be serialised into json or if the file can't be written to.
#[inline]
pub fn as_json<T: Serialize>(instance: &T, path: &Path) -> Result<(), Error> {
    let s = to_string(instance)?;
    Ok(write(path, s)?)
}

impl<T: Debug + Numeric> Save for Array2<T> {
    #[inline]
    fn save(&self, path: &Path) -> Result<(), Error> {
        let mut file = netcdf::create(path).expect("Unable to create file.");

        let shape = self.shape();

        let dim1_name = "x";
        file.add_dimension(dim1_name, shape[X])?;
        let dim2_name = "y";
        file.add_dimension(dim2_name, shape[Y])?;

        let mut var = file.add_variable::<T>("data", &[dim1_name, dim2_name])?;
        var.put_values(self.as_slice().unwrap(), None, None)?;

        Ok(())
    }
}
