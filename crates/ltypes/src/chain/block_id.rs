use super::BlockHash;
use super::BlockHeight;

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

/// Unique block identifier scoped by chain.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum BlockID {
    /// Digest over a chain's block data structure.
    BlockHash(BlockHash),

    /// Monotonically increasing chain height.
    BlockHeight(BlockHeight),
}
