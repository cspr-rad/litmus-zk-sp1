use crate::binary::utils::{CodecError, Decode, Encode};
use ltypes::chain::{Block, BlockV2, BlockV2Body, BlockV2Header};

// ------------------------------------------------------------------------
// Type: Block.
// ------------------------------------------------------------------------

impl Decode for Block {
    #[inline(always)]
    fn decode(_: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        unimplemented!();
    }
}

impl Encode for Block {
    fn get_encoded_size(&self) -> usize {
        unimplemented!("conversion from vec of bytes to domain type BlockV2");
    }

    fn write_bytes(&self, _: &mut Vec<u8>) -> Result<(), CodecError> {
        unimplemented!("Encode for Block:write_bytes");
    }
}

// ------------------------------------------------------------------------
// Type: BlockV2.
// ------------------------------------------------------------------------

impl Decode for BlockV2 {
    #[inline(always)]
    fn decode(_: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        unimplemented!();
    }
}

impl Encode for BlockV2 {
    fn get_encoded_size(&self) -> usize {
        unimplemented!("conversion from vec of bytes to domain type BlockV2");
    }

    fn write_bytes(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        self.hash().write_bytes(writer).unwrap();
        self.header().write_bytes(writer).unwrap();
        self.body().write_bytes(writer).unwrap();
        Ok(())
    }
}

// ------------------------------------------------------------------------
// Type: BlockV2Body.
// ------------------------------------------------------------------------

impl Decode for BlockV2Body {
    #[inline(always)]
    fn decode(_: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        unimplemented!();
    }
}

impl Encode for BlockV2Body {
    fn get_encoded_size(&self) -> usize {
        unimplemented!("conversion from vec of bytes to domain type BlockV2");
    }

    fn write_bytes(&self, _: &mut Vec<u8>) -> Result<(), CodecError> {
        unimplemented!("Encode for BlockV2Body:write_bytes");
    }
}

// ------------------------------------------------------------------------
// Type: BlockV2Header.
// ------------------------------------------------------------------------

impl Decode for BlockV2Header {
    #[inline(always)]
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
        Ok(())

        // result.extend_from_slice(self.parent_hash().inner().as_slice());
        // result.extend_from_slice(self.state_root_hash().as_slice());
        // result.extend_from_slice(self.body_hash().as_slice());
        // result.extend_from_slice(
        //     u8::from(self.random_bit().to_owned())
        //         .to_le_bytes()
        //         .as_slice(),
        // );
        // result.extend_from_slice(self.accumulated_seed().as_slice());
        // // result.extend_from_slice(self.era_end().hash(state););
        // result.extend_from_slice(self.timestamp().inner().to_le_bytes().as_slice());
        // result.extend_from_slice(self.era_id().inner().to_le_bytes().as_slice());
        // result.extend_from_slice(self.height().inner().to_le_bytes().as_slice());
        // // result.extend_from_slice(self.protocol_version());
        // result.extend_from_slice(self.proposer().as_slice());
        // result.extend_from_slice(self.current_gas_price().to_le_bytes().as_slice());
        // // result.extend_from_slice(self.last_switch_block_hash().unwrap().inner().as_slice());
    }
}
