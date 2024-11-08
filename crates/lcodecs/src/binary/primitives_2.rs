use super::utils::{Error as CodecError, ToBytes};
use lutils::bites::Byte;

/// Encoded size: unit.
const ENCODED_SIZE_UNIT: usize = 0;

impl ToBytes for () {
    fn to_bytes(&self) -> Result<Vec<Byte>, CodecError> {
        Ok(Vec::new())
    }

    fn serialized_length(&self) -> usize {
        ENCODED_SIZE_UNIT
    }
}
