use crate::crypto::DigestBytesRaw;

/// Digest over a chain's block data structure.
#[derive(Clone, Debug)]
pub struct BlockHash(DigestBytesRaw);

/// Constructors.
impl BlockHash {
    pub fn new(digest: DigestBytesRaw) -> Self {
        Self(digest)
    }
}

/// Accessors.
impl BlockHash {
    pub fn inner(&self) -> &DigestBytesRaw {
        &self.0
    }
}
