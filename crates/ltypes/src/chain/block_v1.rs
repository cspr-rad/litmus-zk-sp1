use super::BlockHash;
use super::BlockV1Body as BlockBody;
use super::BlockV1Header as BlockHeader;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BlockV1 {
    pub hash: BlockHash,
    pub header: BlockBody,
    pub body: BlockHeader,
}
