use crate::utils::bites::{Bytes32, VecOfBytes, SIZE_32 as SIZE_BYTES_32};

// Raw bytes that represent a digest of some type.
pub type DigestBytesRaw = Bytes32;

// Fixed size byte array representation of a digest.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct DigestBytes(Bytes32);

// Constructors.
impl DigestBytes {
    pub fn new(inner: Bytes32) -> Self {
        Self(inner)
    }

    pub fn new_blake2b(data: VecOfBytes) -> Self {
        use blake2::{
            digest::{Update, VariableOutput},
            Blake2bVar,
        };

        let mut hasher = Blake2bVar::new(SIZE_BYTES_32).unwrap();
        hasher.update(&data);
        let mut buffer = [0u8; SIZE_BYTES_32];
        hasher.finalize_variable(&mut buffer).unwrap();

        Self(buffer)
    }
}

// Convertors.
impl DigestBytes {
    pub fn to_vec(&self) -> VecOfBytes {
        self.0.to_vec()
    }
}

impl AsRef<DigestBytes> for DigestBytes {
    fn as_ref(&self) -> &DigestBytes {
        self
    }
}

impl From<&str> for DigestBytes {
    fn from(value: &str) -> Self {
        DigestBytes(
            hex::decode(value)
                .expect("invalid value")
                .try_into()
                .unwrap(),
        )
    }
}

impl From<String> for DigestBytes {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

impl From<&[u8]> for DigestBytes {
    fn from(value: &[u8]) -> Self {
        Self(value.try_into().expect("copy"))
    }
}
