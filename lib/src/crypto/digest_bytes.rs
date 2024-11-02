use crate::utils::bites::{Byte, Bytes32, SIZE_32 as SIZE_BYTES_32};

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

// Fixed size byte array representation of a digest.
pub type DigestBytes = Bytes32;

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl DigestBytes {
    pub fn new_blake2b(data: Vec<Byte>) -> Self {
        use blake2::{
            digest::{Update, VariableOutput},
            Blake2bVar,
        };

        let mut hasher = Blake2bVar::new(SIZE_BYTES_32).unwrap();
        hasher.update(&data);
        let mut buffer = [0u8; SIZE_BYTES_32];
        hasher.finalize_variable(&mut buffer).unwrap();

        Self::new(buffer)
    }
}
