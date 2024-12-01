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
    fn get_encoded_size(&self) -> usize {
        unimplemented!("conversion from vec of bytes to domain type BlockV2");
    }

    fn to_bytes(&self) -> Result<Vec<u8>, CodecError> {
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
    fn get_encoded_size(&self) -> usize {
        unimplemented!("conversion from vec of bytes to domain type BlockV2");
    }

    fn to_bytes(&self) -> Result<Vec<u8>, CodecError> {
        let mut result: Vec<u8> = Vec::<u8>::new();

        // self.hash.write_bytes(writer)?;
        // self.header.write_bytes(writer)?;
        // self.body.write_bytes(writer)

        // self.hash().write_bytes(writer);

        result.extend(self.hash().to_bytes().unwrap());
        result.extend(self.header().to_bytes().unwrap());
        result.extend(self.body().to_bytes().unwrap());

        Ok(result)
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
    fn get_encoded_size(&self) -> usize {
        unimplemented!("conversion from vec of bytes to domain type BlockV2");
    }

    fn to_bytes(&self) -> Result<Vec<u8>, CodecError> {
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
    fn get_encoded_size(&self) -> usize {
        unimplemented!("conversion from vec of bytes to domain type BlockV2");
    }

    fn to_bytes(&self) -> Result<Vec<u8>, CodecError> {
        let mut result: Vec<u8> = Vec::<u8>::new();

        result.extend(self.parent_hash().to_bytes().unwrap());
        result.extend(self.state_root_hash().to_bytes().unwrap());
        result.extend(self.body_hash().to_bytes().unwrap());

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

        Ok(result)
    }
}
