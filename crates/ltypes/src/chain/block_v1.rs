use super::BlockHash;
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

// Block (v1).
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub struct Block {
    /// Information pertaining to vm + consensus.
    pub body: BlockBody,

    /// Digest over block body + header.
    pub hash: BlockHash,

    /// Block meta data.
    pub header: BlockHeader,
}

// Block (v1) body.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub struct BlockBody {}

// Block (v1) body header.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub struct BlockHeader {
    /// The parent block's hash.
    pub parent_hash: BlockHash,
}

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl Block {
    pub fn new(body: BlockBody, hash: BlockHash, header: BlockHeader) -> Self {
        // TODO: validate inputs.
        Self { body, hash, header }
    }
}
