use lutils::bites::Byte;
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

/// Codec error types.
#[derive(Copy, Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[repr(u8)]
#[non_exhaustive]
pub enum CodecError {
    /// Early end of stream while deserializing.
    EarlyEndOfStream = 0,
    /// Formatting error while deserializing.
    Formatting,
    /// Not all input bytes were consumed in [`deserialize`].
    LeftOverBytes,
    /// Out of memory error.
    OutOfMemory,
    /// No serialized representation is available for a value.
    NotRepresentable,
    /// Exceeded a recursion depth limit.
    ExceededRecursionDepth,
}

/// Trait to be implemented by types which can be encoded to a `Vec<Byte>`.
pub trait Encode {
    /// Serializes `&self` to a `Vec<Byte>`.
    fn to_bytes(&self) -> Result<Vec<Byte>, CodecError>;

    /// Consumes `self` and serializes to a `Vec<u8>`.
    fn into_bytes(self) -> Result<Vec<Byte>, CodecError>
    where
        Self: Sized,
    {
        self.to_bytes()
    }

    /// Returns the length of the `Vec<u8>` which would be returned from a successful call to
    /// `to_bytes()` or `into_bytes()`.  The data is not actually serialized, so this call is
    /// relatively cheap.
    fn serialized_length(&self) -> usize;

    /// Writes `&self` into a mutable `writer`.
    fn write_bytes(&self, writer: &mut Vec<Byte>) -> Result<(), CodecError> {
        writer.extend(self.to_bytes()?);
        Ok(())
    }
}

/// Returns a `Vec<Byte>` initialized with sufficient capacity to hold `to_be_serialized` after
/// serialization, or an error if the capacity would exceed `u32::MAX`.
pub fn allocate_buffer<T: Encode>(to_be_serialized: &T) -> Result<Vec<Byte>, CodecError> {
    let serialized_length = to_be_serialized.serialized_length();
    if serialized_length > u32::MAX as usize {
        return Err(CodecError::OutOfMemory);
    }
    Ok(Vec::with_capacity(serialized_length))
}

/// Returns a `Vec<u8>` initialized with sufficient capacity to hold `to_be_serialized` after
/// serialization.
pub fn unchecked_allocate_buffer<T: Encode>(to_be_serialized: &T) -> Vec<Byte> {
    let serialized_length = to_be_serialized.serialized_length();
    Vec::with_capacity(serialized_length)
}
