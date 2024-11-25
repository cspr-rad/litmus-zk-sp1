use super::super::utils::{CodecError, Decode, Encode};
use ltypes::primitives::time::Timestamp;

// ------------------------------------------------------------------------
// Type: Digest.
// ------------------------------------------------------------------------

impl Decode for Timestamp {
    #[inline(always)]
    fn from_bytes(_: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        unimplemented!();
    }
}

impl Encode for Timestamp {
    fn to_bytes(&self) -> Result<Vec<u8>, CodecError> {
        match self {
            unimplemented!()
        }
    }

    fn get_encoded_size(&self) -> usize {
        match self {
            unimplemented!()
        }
    }

    fn write_bytes(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        unimplemented!()
    }
}
