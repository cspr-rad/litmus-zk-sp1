use crate::crypto::DigestBytes;

/// Digest over a chain's block data structure.
#[derive(Clone, Debug)]
pub struct BlockHash(DigestBytes);

/// Constructors.
impl BlockHash {
    pub fn new(digest: DigestBytes) -> Self {
        Self(digest)
    }
}

/// Accessors.
impl BlockHash {
    pub fn inner(&self) -> &DigestBytes {
        &self.0
    }
}
