use lcrypto::Digest;
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

/// Digest over a chain's block data structure.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub struct BlockHash(Digest);

/// Monotonically increasing chain height.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub struct BlockHeight(u64);

/// Unique block identifier scoped by chain.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub enum BlockID {
    /// Digest over a chain's block data structure.
    BlockHash(BlockHash),

    /// Monotonically increasing chain height.
    BlockHeight(BlockHeight),
}

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl BlockHash {
    /// Constructor: new [`BlockHash`] instance.
    pub fn new(digest: Digest) -> Self {
        Self(digest)
    }
}

impl BlockHeight {
    /// Constructor: new [`BlockHeight`] instance.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }
}

// ------------------------------------------------------------------------
// Accessors.
// ------------------------------------------------------------------------

impl BlockHash {
    pub fn inner(&self) -> &Digest {
        &self.0
    }
}

impl BlockHeight {
    pub fn inner(&self) -> u64 {
        self.0
    }
}
