//! File re-direction implementation.

#[derive(Debug, serde::Deserialize)]
pub enum Oop<T> {
    /// Path to file.
    File(String),
    /// Direct value.
    Here(T),
}

use crate::Load;
impl<T> Load for Oop<T>
where
    for<'de> T: serde::Deserialize<'de>,
{
    #[inline]
    fn load(path: &std::path::Path) -> std::result::Result<Self, crate::Error> {
        crate::from_json(path)
    }
}
