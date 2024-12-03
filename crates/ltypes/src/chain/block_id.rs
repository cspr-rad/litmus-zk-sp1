use crate::crypto::Digest;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Debug;

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
    pub const fn new(height: u64) -> Self {
        Self(height)
    }
}

impl BlockID {
    /// Constructor: new [`BlockID`] instance.
    pub fn new_hash(digest: Digest) -> Self {
        Self::BlockHash(BlockHash::new(digest))
    }

    /// Constructor: new [`BlockID`] instance.
    pub fn new_height(height: u64) -> Self {
        Self::BlockHeight(BlockHeight::new(height))
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

// ------------------------------------------------------------------------
// Traits.
// ------------------------------------------------------------------------

impl fmt::Display for BlockHash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BLOCK-HASH:{}", self.inner())
    }
}

impl fmt::Display for BlockHeight {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.inner())
    }
}

impl From<&str> for BlockHash {
    fn from(value: &str) -> Self {
        Self::new(Digest::from(value))
    }
}

impl From<&[u8]> for BlockHash {
    fn from(value: &[u8]) -> Self {
        Self::new(Digest::from(value))
    }
}

impl From<Vec<u8>> for BlockHash {
    fn from(value: Vec<u8>) -> Self {
        Self::from(value.as_slice())
    }
}

impl From<&Vec<u8>> for BlockHash {
    fn from(value: &Vec<u8>) -> Self {
        Self::from(value.as_slice())
    }
}
