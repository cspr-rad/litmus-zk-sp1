use super::digest_bytes::DigestBytes;
use crate::utils::bites::{Byte, Bytes32};

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

/// Digest scoped by hashing algo type.
#[derive(Clone, Debug)]
pub enum Digest {
    BLAKE2B(DigestBytes),
}

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl Digest {
    pub fn new_blake2b(data: Bytes32) -> Self {
        Self::BLAKE2B(DigestBytes::new_blake2b(data))
    }
}

// ------------------------------------------------------------------------
// Methods.
// ------------------------------------------------------------------------

impl Digest {
    /// Verifies a digest over passed data.
    ///
    /// # Arguments
    ///
    /// * `data` - Data over which to generate a digest.
    ///
    pub fn verify(&self, data: Bytes32) {
        match self {
            Digest::BLAKE2B(inner) => {
                assert_eq!(inner, &DigestBytes::new_blake2b(data))
            }
        }
    }
}

// ------------------------------------------------------------------------
// Traits.
// ------------------------------------------------------------------------

impl From<&Digest> for DigestBytes {
    fn from(x: &Digest) -> Self {
        match x {
            Digest::BLAKE2B(inner) => inner.to_owned(),
        }
    }
}
