use super::block_hash::BlockHash;
use super::block_v1_body::BlockV1Body as BlockBody;
use super::block_v1_header::BlockV1Header as BlockHeader;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BlockV1 {
    pub hash: BlockHash,
    pub header: BlockBody,
    pub body: BlockHeader,
}
