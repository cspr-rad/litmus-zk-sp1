use lutils::bites::{Byte, Bytes32};

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

/// Digest scoped by hashing algo type.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Digest {
    BLAKE2B(Bytes32),
}

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl Digest {
    /// Constructor: returns a new blake2b digest over passed data.
    ///
    /// # Arguments
    ///
    /// * `data` - Data against which to generate a blake2b digest.
    ///
    pub fn new(data: Vec<Byte>) -> Self {
        Self::new_blake2b(data)
    }

    /// Constructor: returns a new blake2b digest over passed data.
    ///
    /// # Arguments
    ///
    /// * `data` - Data against which to generate a blake2b digest.
    ///
    pub fn new_blake2b(data: Vec<Byte>) -> Self {
        use blake2::{
            digest::{Update, VariableOutput},
            Blake2bVar,
        };

        let mut hasher = Blake2bVar::new(Bytes32::len()).unwrap();
        hasher.update(&data);
        let mut buffer = Bytes32::default().data();
        hasher.finalize_variable(&mut buffer).unwrap();

        Self::BLAKE2B(Bytes32::new(buffer))
    }
}

// ------------------------------------------------------------------------
// Accessors.
// ------------------------------------------------------------------------

impl Digest {
    // Returns underlying byte array.
    pub fn as_slice(&self) -> &[Byte] {
        match self {
            Digest::BLAKE2B(inner) => inner.as_slice(),
        }
    }
}

// ------------------------------------------------------------------------
// Methods.
// ------------------------------------------------------------------------

impl Digest {
    /// Verifies digest against passed data.
    ///
    /// # Arguments
    ///
    /// * `data` - Data against which to verify digest.
    ///
    pub fn verify(&self, data: Vec<Byte>) {
        match self {
            Digest::BLAKE2B(_) => {
                assert_eq!(self, &Digest::new_blake2b(data));
            }
        }
    }
}
