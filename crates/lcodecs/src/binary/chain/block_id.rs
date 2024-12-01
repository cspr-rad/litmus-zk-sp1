use crate::binary::utils::{CodecError, Decode, Encode};
use ltypes::chain::{BlockHash, BlockHeight, EraId};

// ------------------------------------------------------------------------
// Codec: BlockHash.
// ------------------------------------------------------------------------

impl Decode for BlockHash {
    fn decode(_: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        unimplemented!();
    }
}

impl Encode for BlockHash {
    fn get_encoded_size(&self) -> usize {
        unimplemented!("conversion from vec of bytes to domain type BlockV2");
    }

    fn write_bytes(&self, _: &mut Vec<u8>) -> Result<(), CodecError> {
        unimplemented!("Encode for BlockHash:write_bytes")
    }
}

// ------------------------------------------------------------------------
// Codec: BlockHeight.
// ------------------------------------------------------------------------

impl Decode for BlockHeight {
    fn decode(_: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        unimplemented!();
    }
}

impl Encode for BlockHeight {
    fn get_encoded_size(&self) -> usize {
        unimplemented!();
    }

    fn write_bytes(&self, _: &mut Vec<u8>) -> Result<(), CodecError> {
        unimplemented!();
    }
}
