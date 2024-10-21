use crate::crypto::DigestBytesRaw;

/// Digest over a chain's block data structure.
pub struct BlockHash(DigestBytesRaw);

/// Monotonically increasing chain height.
pub struct BlockHeight(u64);

/// Unique block identifier scoped by chain.
pub enum BlockID {
    /// Digest over a chain's block data structure.
    BlockHash(BlockHash),

    /// Monotonically increasing chain height.
    BlockHeight(BlockHeight),
}
