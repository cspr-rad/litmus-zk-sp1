use super::constants;
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
pub(crate) fn allocate_buffer<T: Encode>(to_be_serialized: &T) -> Result<Vec<Byte>, CodecError> {
    let serialized_length = to_be_serialized.get_encoded_size();
    if serialized_length > u32::MAX as usize {
        return Err(CodecError::OutOfMemory);
    }
    Ok(Vec::with_capacity(serialized_length))
}

/// Serializes a slice of bytes with a length prefix.
///
/// This function is serializing a slice of bytes with an addition of a 4 byte length prefix.
///
/// For safety you should prefer to use [`vec_u8_to_bytes`]. For efficiency reasons you should also
/// avoid using serializing Vec<u8>.
pub(crate) fn encode_byte_slice(bytes: &[Byte]) -> Result<Vec<Byte>, CodecError> {
    let serialized_length = get_encoded_size_of_byte_slice(bytes);
    let mut vec = Vec::with_capacity(serialized_length);
    let length_prefix: u32 = bytes
        .len()
        .try_into()
        .map_err(|_| CodecError::NotRepresentable)?;
    let length_prefix_bytes = length_prefix.to_le_bytes();
    vec.extend_from_slice(&length_prefix_bytes);
    vec.extend_from_slice(bytes);
    Ok(vec)
}

/// Returns serialized length of serialized slice of bytes.
///
/// This function adds a length prefix in the beginning.
pub(crate) fn get_encoded_size_of_byte_slice(bytes: &[u8]) -> usize {
    constants::ENCODED_SIZE_U32 + bytes.len()
}

pub(crate) fn get_encoded_size_of_byte_vec(vec: &Vec<u8>) -> usize {
    get_encoded_size_of_byte_slice(vec.as_slice())
}

/// Safely splits slice at given point.
pub(crate) fn safe_split_at(bytes: &[Byte], n: usize) -> Result<(&[Byte], &[Byte]), CodecError> {
    if n > bytes.len() {
        Err(CodecError::EarlyEndOfStream)
    } else {
        Ok(bytes.split_at(n))
    }
}

/// Asserts that `t` can be serialized and when deserialized back into an instance `T` compares
/// equal to `t`.
///
/// Also asserts that `t.serialized_length()` is the same as the actual number of bytes of the
/// serialized `t` instance.
#[cfg(any(feature = "testing", test))]
#[track_caller]
pub fn test_serialization_roundtrip<T>(t: &T)
where
    T: Encode + Decode + PartialEq,
{
    let encoded = Encode::to_bytes(t).expect("Unable to serialize data");
    assert_eq!(
        encoded.len(),
        t.get_encoded_size(),
        "\nLength of serialized data: {},\nserialized_length() yielded: {},\n t is XXXXX",
        encoded.len(),
        t.get_encoded_size(),
    );

    let mut written_bytes = vec![];
    t.write_bytes(&mut written_bytes)
        .expect("Unable to serialize data via write_bytes");
    assert_eq!(encoded, written_bytes);

    // let decoded = decode_from_slice(&serialized).expect("Unable to deserialize data");
    // assert_eq!(*t, decoded);

    // let decoded = decode::<T>(encoded).expect("Unable to deserialize data");
    // assert_eq!(*t, decoded);
}

/// Returns a `Vec<u8>` initialized with sufficient capacity to hold `to_be_serialized` after
/// serialization.
pub(crate) fn unchecked_allocate_buffer<T: Encode>(to_be_serialized: &T) -> Vec<Byte> {
    let serialized_length = to_be_serialized.get_encoded_size();
    Vec::with_capacity(serialized_length)
}

pub(crate) fn write_byte_slice(bytes: &[Byte], writer: &mut Vec<Byte>) -> Result<(), CodecError> {
    let length_32: u32 = bytes
        .len()
        .try_into()
        .map_err(|_| CodecError::NotRepresentable)?;
    writer.extend_from_slice(&length_32.to_le_bytes());
    writer.extend_from_slice(bytes);
    Ok(())
}
