// Length of fixed byte digest array.
const LENGTH_OF_DIGEST: usize = 32;

// Raw digest bytes.
pub type DigestBytes = [u8; LENGTH_OF_DIGEST];

/// Digest bytes scoped by hashing algo type.
#[derive(Clone, Debug)]
pub enum Digest {
    BLAKE2B(DigestBytes),
}

impl Digest {
    fn to_bytes(&self) -> &DigestBytes {
        match self {
            Self::BLAKE2B(digest) => digest,
        }
    }
}

// impl From<&Digest> for DigestBytes {
//     fn from(x: &Digest) -> Self {
//         match x {
//             Digest::BLAKE2B(digest) => digest,
//         }
//     }
// }

impl Digest {
    /// Verifies a digest over passed data.
    ///
    /// # Arguments
    ///
    /// * `data` - Data over which to generate a digest.
    ///
    pub fn verify(&self, data: Vec<u8>) {
        // let f: &DigestBytes = self.into();
        // let f = self.into<DigestBytesRaw>();

        match self {
            Digest::BLAKE2B(digest) => {
                assert_eq!(digest, get_blake2b(data).to_bytes().as_slice())
            }
        }
    }
}

/// Returns a blake2b digest over passed data.
///
/// # Arguments
///
/// * `data` - Data over which to generate a digest.
///
fn get_blake2b(data: Vec<u8>) -> Digest {
    use blake2::{
        digest::{Update, VariableOutput},
        Blake2bVar,
    };

    let mut hasher = Blake2bVar::new(LENGTH_OF_DIGEST).unwrap();
    hasher.update(&data);
    let mut buffer = [0u8; LENGTH_OF_DIGEST];
    hasher.finalize_variable(&mut buffer).unwrap();

    Digest::BLAKE2B(buffer)
}
