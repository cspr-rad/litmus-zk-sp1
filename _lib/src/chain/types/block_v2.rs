use super::block_hash::BlockHash;
use super::block_v2_body::BlockV2Body as BlockBody;
use super::block_v2_header::BlockV2Header as BlockHeader;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BlockV2 {
    pub hash: BlockHash,
    pub header: BlockBody,
    pub body: BlockHeader,
}
