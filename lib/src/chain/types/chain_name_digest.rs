use crate::crypto::DigestBytesRaw;

/// Digest over a chain's name.
#[derive(Clone, Debug)]
pub struct ChainNameDigest(DigestBytesRaw);

/// Constructors.
impl ChainNameDigest {
    pub fn new(digest: DigestBytesRaw) -> Self {
        Self(digest)
    }

    pub fn new_from_chain_name(name: &str) -> Self {
        ChainNameDigest(Digest::hash(name.as_bytes()))
    }
}

/// Accessors.
impl ChainNameDigest {
    pub fn inner(&self) -> &DigestBytesRaw {
        &self.0
    }
}
