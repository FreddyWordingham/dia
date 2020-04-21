//! Error enumeration.

use std::fmt::{Debug, Formatter};

/// Error enumeration.
pub enum Error {
    /// File loading error.
    Load(std::io::Error),
    /// Reading error.
    Read(json5::Error),
    /// Writing error.
    Write(serde_json::Error),
    /// Environment variable error.
    EnvVar(std::env::VarError),
}

impl From<std::io::Error> for Error {
    #[inline]
    fn from(e: std::io::Error) -> Self {
        Self::Load(e)
    }
}

impl From<json5::Error> for Error {
    #[inline]
    fn from(e: json5::Error) -> Self {
        Self::Read(e)
    }
}

impl From<serde_json::Error> for Error {
    #[inline]
    fn from(e: serde_json::Error) -> Self {
        Self::Write(e)
    }
}

impl From<std::env::VarError> for Error {
    #[inline]
    fn from(e: std::env::VarError) -> Self {
        Self::EnvVar(e)
    }
}

// #![feature(try_trait)]
// impl From<std::option::NoneError> for Error {
//     #[inline]
//     fn from(_e: std::option::NoneError) -> Self {
//         Self::None()
//     }
// }

impl Debug for Error {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        write!(
            fmt,
            "{}",
            match self {
                Self::Load { .. } => "Loading error.",
                Self::Read { .. } => "Reading error.",
                Self::Write { .. } => "Writing error.",
                Self::EnvVar { .. } => "Environment variable missing error.",
            }
        )
    }
}
