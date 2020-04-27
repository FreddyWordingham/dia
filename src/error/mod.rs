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
    /// Writing error.
    Write(serde_json::Error),
    /// Environment variable error.
    EnvVar(std::env::VarError),
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
impl_from_for_err!(Self::Write, serde_json::Error);
impl_from_for_err!(Self::EnvVar, std::env::VarError);

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
                Self::Write { .. } => "Writing",
                Self::EnvVar { .. } => "Environment variable missing",
            },
            match self {
                Self::Load { 0: err } => format!("{:?}", err),
                Self::Read { 0: err } => format!("{:?}", err),
                Self::ParseInt { 0: err } => format!("{:?}", err),
                Self::ParseFloat { 0: err } => format!("{:?}", err),
                Self::Write { 0: err } => format!("{:?}", err),
                Self::EnvVar { 0: err } => format!("{:?}", err),
            }
        )
    }
}
