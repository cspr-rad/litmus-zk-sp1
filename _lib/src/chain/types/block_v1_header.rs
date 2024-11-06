use super::block_hash::BlockHash;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BlockV1Header {
    /// The parent block's hash.
    pub(super) parent_hash: BlockHash,
}
