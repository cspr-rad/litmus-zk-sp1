use crate::binary::utils::{CodecError, Decode, Encode};
use ltypes::chain::{Block, BlockV2, BlockV2Body, BlockV2Header};

// ------------------------------------------------------------------------
// Type: Block.
// ------------------------------------------------------------------------

impl Decode for Block {
    #[inline(always)]
    fn from_bytes(_: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        unimplemented!();
    }
}

impl Encode for Block {
    fn to_bytes(&self) -> Result<Vec<u8>, CodecError> {
        unimplemented!("conversion from vec of bytes to domain type BlockV2");
    }

    fn get_encoded_size(&self) -> usize {
        unimplemented!("conversion from vec of bytes to domain type BlockV2");
    }
}

// ------------------------------------------------------------------------
// Type: BlockV2.
// ------------------------------------------------------------------------

impl Decode for BlockV2 {
    #[inline(always)]
    fn from_bytes(_: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        unimplemented!();
    }
}

impl Encode for BlockV2 {
    fn to_bytes(&self) -> Result<Vec<u8>, CodecError> {
        unimplemented!("conversion from vec of bytes to domain type BlockV2");
    }

    fn get_encoded_size(&self) -> usize {
        unimplemented!("conversion from vec of bytes to domain type BlockV2");
    }
}

// ------------------------------------------------------------------------
// Type: BlockV2Body.
// ------------------------------------------------------------------------

impl Decode for BlockV2Body {
    #[inline(always)]
    fn from_bytes(_: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        unimplemented!();
    }
}

impl Encode for BlockV2Body {
    fn to_bytes(&self) -> Result<Vec<u8>, CodecError> {
        unimplemented!("conversion from vec of bytes to domain type BlockV2");
    }

    fn get_encoded_size(&self) -> usize {
        unimplemented!("conversion from vec of bytes to domain type BlockV2");
    }
}

// ------------------------------------------------------------------------
// Type: BlockV2Header.
// ------------------------------------------------------------------------

impl Decode for BlockV2Header {
    #[inline(always)]
    fn from_bytes(_: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        unimplemented!();
    }
}

impl Encode for BlockV2Header {
    fn to_bytes(&self) -> Result<Vec<u8>, CodecError> {
        unimplemented!("conversion from vec of bytes to domain type BlockV2");
    }

    fn get_encoded_size(&self) -> usize {
        unimplemented!("conversion from vec of bytes to domain type BlockV2");
    }
}
