use super::super::{
    constants,
    utils::{CodecError, Decode, Encode},
};
use ltypes::{
    chain::{
        Block, BlockHash, BlockHeight, BlockV1, BlockV1Body, BlockV1Header, BlockV2, BlockV2Body,
        BlockV2Header, EraEndV2, EraId, ProtocolVersion,
    },
    crypto::{Digest, PublicKey},
    primitives::Timestamp,
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
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (version_tag, bytes) = u8::decode(bytes).unwrap();
        let (block, bytes) = match version_tag {
            TAG_BLOCK_V1 => {
                let (inner, bytes) = BlockV1::decode(bytes).unwrap();
                (Block::new_v1(inner), bytes)
            }
            TAG_BLOCK_V2 => {
                let (inner, bytes) = BlockV2::decode(bytes).unwrap();
                (Block::new_v2(inner), bytes)
            }
            _ => panic!("Invalid block version tag"),
        };

        Ok((block, bytes))
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
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (inner, remainder) = Digest::decode(bytes).unwrap();

        Ok((BlockHash::new(inner), remainder))
    }
}

impl Encode for BlockHash {
    fn get_encoded_size(&self) -> usize {
        self.inner().get_encoded_size()
    }

    fn write_bytes(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        self.inner().write_bytes(writer).unwrap();

        Ok(())
    }
}

// ------------------------------------------------------------------------
// Codec: BlockHeight.
// ------------------------------------------------------------------------

impl Decode for BlockHeight {
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (inner, bytes) = u64::decode(&bytes).unwrap();

        Ok((Self::new(inner), &bytes))
    }
}

impl Encode for BlockHeight {
    fn get_encoded_size(&self) -> usize {
        self.inner().get_encoded_size()
    }

    fn write_bytes(&self, writer: &mut Vec<u8>) -> Result<(), CodecError> {
        self.inner().write_bytes(writer).unwrap();

        Ok(())
    }
}

// ------------------------------------------------------------------------
// Codec: BlockV1.
// ------------------------------------------------------------------------

impl Decode for BlockV1 {
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (hash, bytes) = BlockHash::decode(bytes).unwrap();
        let (header, bytes) = BlockV1Header::decode(bytes).unwrap();
        let (body, bytes) = BlockV1Body::decode(bytes).unwrap();

        Ok((BlockV1::new(body, hash, header), bytes))
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
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (hash, bytes) = BlockHash::decode(bytes).unwrap();
        let (header, bytes) = BlockV2Header::decode(bytes).unwrap();
        let (body, bytes) = BlockV2Body::decode(bytes).unwrap();

        Ok((BlockV2::new(body, hash, header), bytes))
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
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        unimplemented!()
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
    fn decode(bytes: &[u8]) -> Result<(Self, &[u8]), CodecError> {
        let (parent_hash, bytes) = BlockHash::decode(bytes).unwrap();
        let (state_root_hash, bytes) = Digest::decode(bytes).unwrap();
        let (body_hash, bytes) = Digest::decode(bytes).unwrap();
        let (random_bit, bytes) = bool::decode(bytes).unwrap();
        let (accumulated_seed, bytes) = Digest::decode(bytes).unwrap();
        let (era_end, bytes) = Option::<EraEndV2>::decode(bytes).unwrap();
        let (timestamp, bytes) = Timestamp::decode(bytes).unwrap();
        let (era_id, bytes) = EraId::decode(bytes).unwrap();
        let (height, bytes) = BlockHeight::decode(bytes).unwrap();
        let (protocol_version, bytes) = ProtocolVersion::decode(bytes).unwrap();
        let (proposer, bytes) = PublicKey::decode(bytes).unwrap();
        let (current_gas_price, bytes) = u8::decode(bytes).unwrap();
        let (last_switch_block_hash, bytes) = Option::<BlockHash>::decode(bytes).unwrap();

        Ok((
            BlockV2Header::new(
                accumulated_seed,
                body_hash,
                current_gas_price,
                era_end,
                era_id,
                height,
                last_switch_block_hash,
                parent_hash,
                proposer,
                protocol_version,
                random_bit,
                state_root_hash,
                timestamp,
            ),
            bytes,
        ))
    }
}

impl Encode for BlockV2Header {
    fn get_encoded_size(&self) -> usize {
        self.parent_hash().get_encoded_size()
            + self.state_root_hash().get_encoded_size()
            + self.body_hash().get_encoded_size()
            + self.random_bit().get_encoded_size()
            + self.accumulated_seed().get_encoded_size()
            + self.era_end().get_encoded_size()
            + self.timestamp().get_encoded_size()
            + self.era_id().get_encoded_size()
            + self.height().get_encoded_size()
            + self.protocol_version().get_encoded_size()
            + self.proposer().get_encoded_size()
            + self.current_gas_price().get_encoded_size()
            + self.last_switch_block_hash().get_encoded_size()
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
