use crate::binary::decode;

use super::super::utils::{CodecError, Decode, Encode};
use ltypes::primitives::time::Timestamp;

// ------------------------------------------------------------------------
// Type: Digest.
// ------------------------------------------------------------------------

impl Decode for Timestamp {
    #[inline(always)]
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (inner, bytes) = u128::from_bytes(&bytes).unwrap();

        Ok((Self::new(inner), &bytes))
    }
}

impl Encode for Timestamp {
    fn to_bytes(&self) -> Result<Vec<u8>, CodecError> {
        unimplemented!();
    }

    fn get_encoded_size(&self) -> usize {
        unimplemented!();
    }

    fn write_bytes(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        unimplemented!();
    }
}
