use super::super::{
    constants,
    utils::{CodecError, Decode, Encode},
};
use ltypes::chain::{
    Block, BlockHash, BlockHeight, BlockV1, BlockV1Body, BlockV1Header, BlockV2, BlockV2Body,
    BlockV2Header,
};

// ------------------------------------------------------------------------
// Constants.
// ------------------------------------------------------------------------

const TAG_BLOCK_V1: u8 = 0;
const TAG_BLOCK_V2: u8 = 1;

// ------------------------------------------------------------------------
// Codec: Block.
// ------------------------------------------------------------------------

impl Decode for Block {
    fn decode(_: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        unimplemented!();
    }
}

impl Encode for Block {
    fn get_encoded_size(&self) -> usize {
        constants::ENCODED_SIZE_U8
            + match self {
                Block::V1(inner) => inner.get_encoded_size(),
                Block::V2(inner) => inner.get_encoded_size(),
            }
    }

    fn write_bytes(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        match self {
            Block::V1(inner) => {
                writer.push(TAG_BLOCK_V1);
                inner.write_bytes(writer).unwrap();
            }
            Block::V2(inner) => {
                writer.push(TAG_BLOCK_V2);
                inner.write_bytes(writer).unwrap();
            }
        }
        Ok(())
    }
}

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

// ------------------------------------------------------------------------
// Codec: BlockV1.
// ------------------------------------------------------------------------

impl Decode for BlockV1 {
    fn decode(_: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        unimplemented!();
    }
}

impl Encode for BlockV1 {
    fn get_encoded_size(&self) -> usize {
        self.hash().get_encoded_size()
            + self.header().get_encoded_size()
            + self.body().get_encoded_size()
    }

    fn write_bytes(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        self.hash().write_bytes(writer).unwrap();
        self.header().write_bytes(writer).unwrap();
        self.body().write_bytes(writer).unwrap();
        Ok(())
    }
}

// ------------------------------------------------------------------------
// Codec: BlockV1Body.
// ------------------------------------------------------------------------

impl Decode for BlockV1Body {
    fn decode(_: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        unimplemented!();
    }
}

impl Encode for BlockV1Body {
    fn get_encoded_size(&self) -> usize {
        unimplemented!();
    }

    fn write_bytes(&self, _: &mut Vec<u8>) -> Result<(), CodecError> {
        unimplemented!();
    }
}

// ------------------------------------------------------------------------
// Codec: BlockV1Header.
// ------------------------------------------------------------------------

impl Decode for BlockV1Header {
    fn decode(_: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        unimplemented!();
    }
}

impl Encode for BlockV1Header {
    fn get_encoded_size(&self) -> usize {
        unimplemented!();
    }

    fn write_bytes(&self, _: &mut Vec<u8>) -> Result<(), CodecError> {
        unimplemented!();
    }
}

// ------------------------------------------------------------------------
// Codec: BlockV2.
// ------------------------------------------------------------------------

impl Decode for BlockV2 {
    fn decode(_: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        unimplemented!();
    }
}

impl Encode for BlockV2 {
    fn get_encoded_size(&self) -> usize {
        self.hash().get_encoded_size()
            + self.header().get_encoded_size()
            + self.body().get_encoded_size()
    }

    fn write_bytes(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        self.hash().write_bytes(writer).unwrap();
        self.header().write_bytes(writer).unwrap();
        self.body().write_bytes(writer).unwrap();
        Ok(())
    }
}

// ------------------------------------------------------------------------
// Codec: BlockV2Body.
// ------------------------------------------------------------------------

impl Decode for BlockV2Body {
    fn decode(_: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        unimplemented!();
    }
}

impl Encode for BlockV2Body {
    fn get_encoded_size(&self) -> usize {
        self.transactions().get_encoded_size() + self.rewarded_signatures().get_encoded_size()
    }

    fn write_bytes(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        self.transactions().write_bytes(writer).unwrap();
        self.rewarded_signatures().write_bytes(writer).unwrap();
        Ok(())
    }
}

// ------------------------------------------------------------------------
// Codec: BlockV2Header.
// ------------------------------------------------------------------------

impl Decode for BlockV2Header {
    fn decode(_: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        unimplemented!();
    }
}

impl Encode for BlockV2Header {
    fn get_encoded_size(&self) -> usize {
        unimplemented!("conversion from vec of bytes to domain type BlockV2");
    }

    fn write_bytes(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        self.parent_hash().write_bytes(writer).unwrap();
        self.state_root_hash().write_bytes(writer).unwrap();
        self.body_hash().write_bytes(writer).unwrap();
        self.random_bit().write_bytes(writer).unwrap();
        self.accumulated_seed().write_bytes(writer).unwrap();
        self.era_end().write_bytes(writer).unwrap();
        self.timestamp().write_bytes(writer).unwrap();
        self.era_id().write_bytes(writer).unwrap();
        self.height().write_bytes(writer).unwrap();
        self.protocol_version().write_bytes(writer).unwrap();
        self.proposer().write_bytes(writer).unwrap();
        self.current_gas_price().write_bytes(writer).unwrap();
        self.last_switch_block_hash().write_bytes(writer).unwrap();
        Ok(())
    }
}
