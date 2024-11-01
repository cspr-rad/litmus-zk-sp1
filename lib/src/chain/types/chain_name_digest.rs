use crate::crypto::Digest;

/// Digest over a chain's name.
#[derive(Clone, Debug)]
pub struct ChainNameDigest(Digest);

/// Constructors.
impl ChainNameDigest {
    pub fn new(digest: Digest) -> Self {
        Self(digest)
    }

    pub fn new_from_chain_name(name: &str) -> Self {
        ChainNameDigest(Digest::hash(name.as_bytes()))
    }
}

/// Accessors.
impl ChainNameDigest {
    pub fn inner(&self) -> &Digest {
        &self.0
    }
}
