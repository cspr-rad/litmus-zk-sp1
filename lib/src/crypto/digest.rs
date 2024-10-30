use super::digest_bytes::DigestBytes;

/// Digest scoped by hashing algo type.
#[derive(Clone, Debug)]
pub enum Digest {
    BLAKE2B(DigestBytes),
}

// Constructors.
impl Digest {
    pub fn new_blake2b(data: Vec<u8>) -> Self {
        Self::BLAKE2B(DigestBytes::new_blake2b(data))
    }
}

// Convertors.
impl From<&Digest> for DigestBytes {
    fn from(x: &Digest) -> Self {
        match x {
            Digest::BLAKE2B(inner) => inner.to_owned(),
        }
    }
}

// Methods.
impl Digest {
    /// Verifies a digest over passed data.
    ///
    /// # Arguments
    ///
    /// * `data` - Data over which to generate a digest.
    ///
    pub fn verify(&self, data: Vec<u8>) {
        match self {
            Digest::BLAKE2B(inner) => {
                assert_eq!(inner, &DigestBytes::new_blake2b(data))
            }
        }
    }
}
