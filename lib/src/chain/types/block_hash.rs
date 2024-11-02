use crate::crypto::Digest;

/// Digest over a chain's block data structure.
#[derive(Clone, Debug)]
pub struct BlockHash(Digest);

/// Constructors.
impl BlockHash {
    pub fn new(digest: Digest) -> Self {
        Self(digest)
    }
}

/// Accessors.
impl BlockHash {
    pub fn inner(&self) -> &Digest {
        &self.0
    }
}
