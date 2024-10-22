// Length of fixed byte digest array.
const LENGTH_OF_DIGEST: usize = 32;

/// Raw digest bytes scoped by hashing algo type.
#[derive(Clone, Debug)]
pub enum DigestBytes {
    BLAKE2B(DigestBytesRaw),
}

// Raw digest bytes.
pub type DigestBytesRaw = [u8; LENGTH_OF_DIGEST];

impl DigestBytes {
    fn as_raw(&self) -> &DigestBytesRaw {
        match self {
            Self::BLAKE2B(digest) => digest,
        }
    }
}

impl DigestBytes {
    /// Verifies a digest over passed data.
    ///
    /// # Arguments
    ///
    /// * `data` - Data over which to generate a digest.
    ///
    #[sp1_derive::cycle_tracker]
    pub fn verify(&self, data: Vec<u8>) {
        verify(self.to_owned(), data);
        // match self {
        //     DigestBytes::BLAKE2B(digest) => {
        //         assert_eq!(digest, get_blake2b(data).inner().as_slice())
        //     }
        // }
    }
}

/// Verifies a digest over passed data.
///
/// # Arguments
///
/// * `data` - Data over which to generate a digest.
/// * `digest` - Digest to be verified.
///
#[sp1_derive::cycle_tracker(ident.name = "dsadas")]
pub fn verify(digest: DigestBytes, data: Vec<u8>) {
    match digest {
        DigestBytes::BLAKE2B(digest) => {
            assert_eq!(digest, get_blake2b(data).as_raw().as_slice())
        }
    }
}

/// Returns a blake2b digest over passed data.
///
/// # Arguments
///
/// * `data` - Data over which to generate a digest.
///
fn get_blake2b(data: Vec<u8>) -> DigestBytes {
    use blake2::{
        digest::{Update, VariableOutput},
        Blake2bVar,
    };

    let mut hasher = Blake2bVar::new(LENGTH_OF_DIGEST).unwrap();
    hasher.update(&data);
    let mut buffer = [0u8; LENGTH_OF_DIGEST];
    hasher.finalize_variable(&mut buffer).unwrap();

    DigestBytes::BLAKE2B(buffer)
}
