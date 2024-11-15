use lutils::bites::{Byte, Bytes32};
use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

/// Digest scoped by hashing algo type.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Digest {
    BLAKE2B(Bytes32),
}

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl Digest {
    /// Constructor: returns an instance hydrated from a sequence of bytes.
    ///
    /// # Arguments
    ///
    /// * `raw_bytes` - A sequence of bytes.
    ///
    pub fn new(raw_bytes: &[Byte]) -> Self {
        assert!(
            raw_bytes.len() == Bytes32::len(),
            "Invalid digest byte array length"
        );

        // Problematic if another hashing algo is introduced.
        Self::BLAKE2B(Bytes32::from(raw_bytes))
    }
}

// ------------------------------------------------------------------------
// Methods.
// ------------------------------------------------------------------------

impl Digest {
    // Returns underlying byte array.
    pub fn as_slice(&self) -> &[Byte] {
        match self {
            Digest::BLAKE2B(inner) => inner.as_slice(),
        }
    }

    /// Returns a blake2b digest over passed data.
    ///
    /// # Arguments
    ///
    /// * `data` - Data against which to generate a blake2b digest.
    ///
    pub fn get_blake2b(data: Vec<Byte>) -> Self {
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

    /// Verifies digest against passed data.
    ///
    /// # Arguments
    ///
    /// * `data` - Data against which to verify digest.
    ///
    pub fn verify(&self, data: Vec<Byte>) {
        match self {
            Digest::BLAKE2B(_) => {
                assert_eq!(self, &Digest::get_blake2b(data));
            }
        }
    }
}

// ------------------------------------------------------------------------
// Traits.
// ------------------------------------------------------------------------

impl From<&str> for Digest {
    fn from(value: &str) -> Self {
        Self::from(hex::decode(value).unwrap())
    }
}

impl From<&[Byte]> for Digest {
    fn from(value: &[Byte]) -> Self {
        Self::new(&value)
    }
}

impl From<Vec<Byte>> for Digest {
    fn from(value: Vec<Byte>) -> Self {
        Self::from(value.as_slice())
    }
}

impl From<&Vec<Byte>> for Digest {
    fn from(value: &Vec<Byte>) -> Self {
        Self::from(value.as_slice())
    }
}

// ------------------------------------------------------------------------
// Traits -> serde.
// ------------------------------------------------------------------------

impl<'de> Deserialize<'de> for Digest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DigestVistor;

        impl<'de> Visitor<'de> for DigestVistor {
            type Value = Digest;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("supported formats: 64 char hex encoded string | 32 byte array")
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                // Problematic if another hashing algo is introduced.
                Ok(Digest::from(v))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                // Problematic if another hashing algo is introduced.
                Ok(Digest::from(v))
            }
        }

        deserializer.deserialize_any(DigestVistor)
    }
}

impl Serialize for Digest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Problematic if another hashing algo is introduced.
        let as_hex = hex::encode(&self.as_slice());

        Ok(serializer.serialize_str(&as_hex).unwrap())
    }
}
