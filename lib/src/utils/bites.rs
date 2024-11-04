use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------
// Constants.
// ------------------------------------------------------------------------

// Sizes of fixed byte digest arrays.
pub const SIZE_32: usize = 32;
pub const SIZE_33: usize = 33;
pub const SIZE_64: usize = 64;

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

// Raw byte.
pub type Byte = u8;

// Generic byte array with constant size of N.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Bytes<const N: usize> {
    pub data: [Byte; N],
}

// Byte array of size 32 - typically a digest | public key.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub type Bytes32 = Bytes<SIZE_32>;

// Byte array of size 33 - typically a public key.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub type Bytes33 = Bytes<SIZE_33>;

// Byte array of size 64 - typically a signature.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub type Bytes64 = Bytes<SIZE_64>;

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl<const N: usize> Bytes<N> {
    // Initializes instance from specific data.
    pub fn new(data: [Byte; N]) -> Self {
        Self { data }
    }
}

// ------------------------------------------------------------------------
// Accessors.
// ------------------------------------------------------------------------

impl<const N: usize> Bytes<N> {
    // Returns underlying byte array.
    pub fn as_slice(&self) -> &[Byte] {
        &self.data
    }

    // Returns underlying byte array mutably.
    pub fn as_mut_slice(&mut self) -> &mut [Byte] {
        &mut self.data
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
    pub fn to_vec(&self) -> Vec<Byte> {
        self.data.to_vec()
    }
}

// ------------------------------------------------------------------------
// Traits.
// ------------------------------------------------------------------------

// Default -> returns default value.
impl<const N: usize> Default for Bytes<N> {
    fn default() -> Self {
        Self::new([0; N])
    }
}

// AsRef -> borrowed self to self.
impl<const N: usize> AsRef<Bytes<N>> for Bytes<N> {
    fn as_ref(&self) -> &Self {
        self
    }
}

// From -> borrowed slice to self.
impl<const N: usize> From<&[Byte]> for Bytes<N> {
    fn from(value: &[Byte]) -> Self {
        Self::new(value[0..N].try_into().unwrap())
    }
}

// From -> owned byte vec to self.
impl<const N: usize> From<Vec<Byte>> for Bytes<N> {
    fn from(value: Vec<Byte>) -> Self {
        Bytes::from(&value)
    }
}

// From -> borrowed vec of bytes -> self.
impl<const N: usize> From<&Vec<Byte>> for Bytes<N> {
    fn from(value: &Vec<Byte>) -> Self {
        Bytes::from(value.as_slice())
    }
}

// From -> borrowed string slice -> self.
impl<const N: usize> From<&str> for Bytes<N> {
    fn from(value: &str) -> Self {
        Self::from(hex::decode(value).unwrap())
    }
}

// From -> owned string -> self.
impl<const N: usize> From<String> for Bytes<N> {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

// From -> borrowed string -> self.
impl<const N: usize> From<&String> for Bytes<N> {
    fn from(value: &String) -> Self {
        Self::from(value.to_owned())
    }
}
