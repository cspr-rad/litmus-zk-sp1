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

/// Trait implemented by types decodeable from a `Vec<Byte>`.
pub trait Decode: Sized {
    /// Decodes slice into instance of `Self`.
    fn from_bytes(bytes: &[Byte]) -> Result<(Self, &[Byte]), CodecError>;

    /// Decodes `Vec<u8>` into instance of `Self`.
    fn from_vec(bytes: Vec<Byte>) -> Result<(Self, Vec<Byte>), CodecError> {
        Self::from_bytes(bytes.as_slice()).map(|(x, remainder)| (x, Vec::from(remainder)))
    }
}

/// Trait implemented by types encodeable as a `Vec<Byte>`.
pub trait Encode {
    /// Encodes `&self` as a `Vec<Byte>`.
    fn to_bytes(&self) -> Result<Vec<Byte>, CodecError>;

    /// Consumes `self` and encodes accordingly.
    fn into_bytes(self) -> Result<Vec<Byte>, CodecError>
    where
        Self: Sized,
    {
        self.to_bytes()
    }

    /// Returns size of `Vec<u8>` returned from call to `encode()`.
    fn get_encoded_size(&self) -> usize;

    /// Writes `&self` into a mutable `writer`.
    fn write_bytes(&self, writer: &mut Vec<Byte>) -> Result<(), CodecError> {
        writer.extend(self.to_bytes()?);
        Ok(())
    }
}

/// Returns a `Vec<Byte>` initialized with sufficient capacity to hold `to_be_serialized` after
/// serialization, or an error if the capacity would exceed `u32::MAX`.
pub fn allocate_buffer<T: Encode>(to_be_serialized: &T) -> Result<Vec<Byte>, CodecError> {
    let serialized_length = to_be_serialized.get_encoded_size();
    if serialized_length > u32::MAX as usize {
        return Err(CodecError::OutOfMemory);
    }
    Ok(Vec::with_capacity(serialized_length))
}

/// Safely splits slice at given point.
pub(crate) fn safe_split_at(bytes: &[Byte], n: usize) -> Result<(&[Byte], &[Byte]), CodecError> {
    if n > bytes.len() {
        Err(CodecError::EarlyEndOfStream)
    } else {
        Ok(bytes.split_at(n))
    }
}

/// Returns a `Vec<u8>` initialized with sufficient capacity to hold `to_be_serialized` after
/// serialization.
pub fn unchecked_allocate_buffer<T: Encode>(to_be_serialized: &T) -> Vec<Byte> {
    let serialized_length = to_be_serialized.get_encoded_size();
    Vec::with_capacity(serialized_length)
}
