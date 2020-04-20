//! Input and output error enumeration.

/// IO error enumeration.
pub enum Error {
    /// File loading error.
    Load(std::io::Error),
    /// Reading error.
    Read(json5::Error),
    /// Writing error.
    Write(serde_json::Error),
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
