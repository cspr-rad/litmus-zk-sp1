use serde::{Deserialize, Serialize};
use std::fmt;

// ------------------------------------------------------------------------
// Constants.
// ------------------------------------------------------------------------

const SIZE_32: usize = 32;
const SIZE_33: usize = 33;
const SIZE_64: usize = 64;
const SIZE_65: usize = 65;

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

// Generic byte array with constant size of N.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct Bytes<const N: usize> {
    #[serde(with = "serde_bytes")]
    data: [u8; N],
}

// Byte array of size 32 - typically a digest | public key.
pub type Bytes32 = Bytes<SIZE_32>;

// Byte array of size 33 - typically a public key.
pub type Bytes33 = Bytes<SIZE_33>;

// Byte array of size 64 - typically a signature.
pub type Bytes64 = Bytes<SIZE_64>;

// Byte array of size 65 - typically a signature prefixed by it's algo type.
pub type Bytes65 = Bytes<SIZE_65>;

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl<const N: usize> Bytes<N> {
    // Initializes instance from specific data.
    pub fn new(data: [u8; N]) -> Self {
        Self { data }
    }
}

// ------------------------------------------------------------------------
// Accessors.
// ------------------------------------------------------------------------

impl<const N: usize> Bytes<N> {
    // Returns reference to underlying byte array.
    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }

    // Returns reference to underlying byte array mutably.
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.data
    }

    // Returns underlying byte array.
    pub fn data(&self) -> [u8; N] {
        self.data
    }

    // Returns length of underlying byte array.
    pub fn len() -> usize {
        N
    }
}

// ------------------------------------------------------------------------
// Methods.
// ------------------------------------------------------------------------

impl<const N: usize> Bytes<N> {
    // Method: Convert data to vec.
    pub fn to_vec(&self) -> Vec<u8> {
        self.data.to_vec()
    }
}

// ------------------------------------------------------------------------
// Traits.
// ------------------------------------------------------------------------

impl<const N: usize> Default for Bytes<N> {
    fn default() -> Self {
        Self::new([0; N])
    }
}

impl<const N: usize> fmt::Display for Bytes<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({}..{})",
            hex::encode(&self.data[0..2]),
            hex::encode(&self.data[&self.data.len() - 2..])
        )
    }
}

impl<const N: usize> AsRef<Bytes<N>> for Bytes<N> {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<const N: usize> From<&[u8]> for Bytes<N> {
    fn from(value: &[u8]) -> Self {
        Self::new(value[0..N].try_into().unwrap())
    }
}

impl<const N: usize> From<Vec<u8>> for Bytes<N> {
    fn from(value: Vec<u8>) -> Self {
        Bytes::from(&value)
    }
}

impl<const N: usize> From<&Vec<u8>> for Bytes<N> {
    fn from(value: &Vec<u8>) -> Self {
        Bytes::from(value.as_slice())
    }
}

impl<const N: usize> From<&str> for Bytes<N> {
    fn from(value: &str) -> Self {
        Self::from(hex::decode(value).unwrap())
    }
}

impl<const N: usize> From<String> for Bytes<N> {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

impl<const N: usize> From<&String> for Bytes<N> {
    fn from(value: &String) -> Self {
        Self::from(value.to_owned())
    }
}
