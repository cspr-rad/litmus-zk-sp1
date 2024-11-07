use super::BlockHash;

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

// Version 1 block.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Block {
    /// Digest over block body + header.
    pub hash: BlockHash,

    /// Information pertaining to vm + consensus.
    pub header: BlockBody,

    /// Block meta data.
    pub body: BlockHeader,
}

// Version 1 block body.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BlockBody {}

// Version 1 block header.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BlockHeader {
    /// The parent block's hash.
    pub parent_hash: BlockHash,
}
