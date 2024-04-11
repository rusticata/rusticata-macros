//! Common traits

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

/// Common trait for structures serialization
pub trait Serialize<O = Vec<u8>> {
    /// Type of serialization error
    type Error;
    /// Try to serialize object
    fn serialize(&self) -> Result<O, Self::Error>;
}
