use super::BlockHash;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BlockV1Header {
    /// The parent block's hash.
    pub parent_hash: BlockHash,
}
