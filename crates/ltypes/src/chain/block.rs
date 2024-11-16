use super::{BlockHash, BlockV1, BlockV2};
use lcrypto::Digest;
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

#[derive(Clone, Debug, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub enum Block {
    #[serde(rename = "Version1")]
    V1(BlockV1),
    #[serde(rename = "Version2")]
    V2(BlockV2),
}

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl Block {
    pub fn new_v1(inner: BlockV1) -> Self {
        Self::V1(inner)
    }

    pub fn new_v2(inner: BlockV2) -> Self {
        Self::V2(inner)
    }
}

// ------------------------------------------------------------------------
// Methods.
// ------------------------------------------------------------------------

impl Block {
    /// Returns a digest to be signed over when commiting to finality.
    pub fn get_bytes_for_finality_signature(&self) -> Vec<u8> {
        match self {
            Block::V1(inner) => inner.get_bytes_for_finality_signature(),
            Block::V2(inner) => inner.get_bytes_for_finality_signature(),
        }
    }
}

// ------------------------------------------------------------------------
// Accessors.
// ------------------------------------------------------------------------

impl Block {
    pub fn hash(&self) -> &BlockHash {
        match self {
            Block::V1(inner) => inner.hash(),
            Block::V2(inner) => inner.hash(),
        }
    }
}
