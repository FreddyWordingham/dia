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
impl_from_for_err!(Self::Write, serde_json::Error);
impl_from_for_err!(Self::EnvVar, std::env::VarError);

// #![feature(try_trait)]
// impl_from_for_err!(Self::None, std::option::NoneError);

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