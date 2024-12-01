use super::super::utils::{CodecError, Decode, Encode};
use ltypes::chain::ValidatorWeight;

// ------------------------------------------------------------------------
// Codec: ValidatorWeight.
// ------------------------------------------------------------------------

impl Decode for ValidatorWeight {
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        unimplemented!()
    }
}

impl Encode for ValidatorWeight {
    fn get_encoded_size(&self) -> usize {
        unimplemented!()
    }

    fn write_bytes(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        unimplemented!()
    }
}
