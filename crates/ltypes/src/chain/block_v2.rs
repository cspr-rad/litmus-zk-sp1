use super::BlockHash;
use super::BlockV2Body as BlockBody;
use super::BlockV2Header as BlockHeader;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct BlockV2 {
    pub hash: BlockHash,
    pub header: BlockBody,
    pub body: BlockHeader,
}
