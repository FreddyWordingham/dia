//! Error enumeration.

use std::fmt::{Debug, Formatter};

/// Error enumeration.
pub enum Error {
    /// File loading error.
    Load(std::io::Error),
    /// Reading error.
    Read(json5::Error),
    /// Integer parsing error.
    ParseInt(std::num::ParseIntError),
    /// Float parsing error.
    ParseFloat(std::num::ParseFloatError),
    /// Json writing error.
    WriteJson(serde_json::Error),
    /// NetCDF writing error.
    WriteNetCDF(netcdf::error::Error),
    /// PNG writing error.
    WritePng(png::EncodingError),
    /// Environment variable error.
    EnvVar(std::env::VarError),
    /// Parallelisation poison.
    Parallel,
    // /// Unexpected None error.
    // None(std::option::NoneError),
}

macro_rules! impl_from_for_err {
    ($enum:path, $error:ty) => {
        impl From<$error> for Error {
            #[inline]
            fn from(e: $error) -> Self {
                $enum(e)
            }
        }
    };
}

impl_from_for_err!(Self::Load, std::io::Error);
impl_from_for_err!(Self::Read, json5::Error);
impl_from_for_err!(Self::ParseInt, std::num::ParseIntError);
impl_from_for_err!(Self::ParseFloat, std::num::ParseFloatError);
impl_from_for_err!(Self::WriteJson, serde_json::Error);
impl_from_for_err!(Self::WriteNetCDF, netcdf::error::Error);
impl_from_for_err!(Self::WritePng, png::EncodingError);
impl_from_for_err!(Self::EnvVar, std::env::VarError);

impl<T> From<std::sync::PoisonError<T>> for Error {
    #[inline]
    fn from(_e: std::sync::PoisonError<T>) -> Self {
        Self::Parallel
    }
}

// TODO: This Requires nightly compiler but would allow us properly to handle unwraping None's as errors.
// // #![feature(try_trait)] // This goes in lib.rs.
// impl_from_for_err!(Self::None, std::option::NoneError);

impl Debug for Error {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        write!(
            fmt,
            "{} error: `{}`",
            match self {
                Self::Load { .. } => "Loading",
                Self::Read { .. } => "Reading",
                Self::ParseInt { .. } => "Integer parsing",
                Self::ParseFloat { .. } => "Float parsing",
                Self::WriteJson { .. } => "JSON writing",
                Self::WriteNetCDF { .. } => "NetCDF writing",
                Self::WritePng { .. } => "PNG writing",
                Self::EnvVar { .. } => "Environment variable missing",
                Self::Parallel { .. } => "Parallelisation poison",
            },
            match self {
                Self::Load { 0: err } => format!("{:?}", err),
                Self::Read { 0: err } => format!("{:?}", err),
                Self::ParseInt { 0: err } => format!("{:?}", err),
                Self::ParseFloat { 0: err } => format!("{:?}", err),
                Self::WriteJson { 0: err } => format!("{:?}", err),
                Self::WriteNetCDF { 0: err } => format!("{:?}", err),
                Self::WritePng { 0: err } => format!("{:?}", err),
                Self::EnvVar { 0: err } => format!("{:?}", err),
                Self::Parallel => format!("parallelisation fail"),
            }
        )
    }
}
