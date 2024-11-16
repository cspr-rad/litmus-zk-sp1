use super::digest::Digest;
use hex;
use lutils::bites::{Byte, Bytes32, Bytes33, Bytes64};
use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

// ------------------------------------------------------------------------
// Constants.
// ------------------------------------------------------------------------

const TAG_ED25519: Byte = 1;
const TAG_SECP256K1: Byte = 2;
const SIG_SIZE: usize = 64;
const VKEY_SIZE_ED25519: usize = 32;
const VKEY_SIZE_RANGE: std::ops::Range<usize> = 33..34;
const VKEY_SIZE_SECP256K1: usize = 33;

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Signature {
    ED25519(Bytes64),
    SECP256K1(Bytes64),
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum VerificationKey {
    ED25519(Bytes32),
    SECP256K1(Bytes33),
}

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl Signature {
    /// Constructor: returns an instance hydrated from a sequence of bytes.
    ///
    /// # Arguments
    ///
    /// * `raw_bytes` - A sequence of bytes.
    ///
    pub fn new(raw_bytes: &[Byte]) -> Self {
        assert!(
            raw_bytes.len() == SIG_SIZE + 1,
            "Invalid signature byte array length"
        );
        match raw_bytes[0] {
            TAG_ED25519 => Self::ED25519(Bytes64::from(&raw_bytes[1..])),
            TAG_SECP256K1 => Self::SECP256K1(Bytes64::from(&raw_bytes[1..])),
            _ => panic!("Unsupported signature key type prefix"),
        }
    }

    /// Constructor: returns a new ed25519 signature.
    ///
    /// # Arguments
    ///
    /// * `sig` - Signature issued by an ed25519 signing key.
    ///
    pub fn new_ed25519(sig: Bytes64) -> Self {
        Signature::ED25519(sig)
    }

    /// Constructor: returns a new secp256k1 signature.
    ///
    /// # Arguments
    ///
    /// * `sig` - Signature issued by a secp256k1 signing key.
    ///
    pub fn new_secp256k1(sig: Bytes64) -> Self {
        Signature::SECP256K1(sig)
    }
}

impl VerificationKey {
    /// Constructor: returns an instance hydrated from a sequence of bytes.
    ///
    /// # Arguments
    ///
    /// * `raw_bytes` - A sequence of bytes.
    ///
    pub fn new(raw_bytes: &[Byte]) -> Self {
        assert!(
            VKEY_SIZE_RANGE.contains(&raw_bytes.len()),
            "Invalid verification key length"
        );
        match raw_bytes[0] {
            TAG_ED25519 => {
                assert!(
                    raw_bytes.len() == VKEY_SIZE_ED25519 + 1,
                    "Invalid verification key length"
                );
                VerificationKey::ED25519(Bytes32::from(raw_bytes[1..33].to_vec()))
            }
            TAG_SECP256K1 => {
                assert!(
                    raw_bytes.len() == VKEY_SIZE_SECP256K1 + 1,
                    "Invalid verification key length"
                );
                VerificationKey::SECP256K1(Bytes33::from(raw_bytes[1..34].to_vec()))
            }
            _ => panic!("Unsupported signature key type prefix"),
        }
    }

    /// Constructor: returns a new ed25519 verification key.
    ///
    /// # Arguments
    ///
    /// * `sig` - Verification key issued by an ed25519 algorithm.
    ///
    pub fn new_ed25519(vk: Bytes32) -> Self {
        VerificationKey::ED25519(vk)
    }

    /// Constructor: returns a new secp256k1 verification key.
    ///
    /// # Arguments
    ///
    /// * `sig` - Verification key issued by an secp256k1 algorithm.
    ///
    pub fn new_secp256k1(vk: Bytes33) -> Self {
        VerificationKey::SECP256K1(vk)
    }
}

// ------------------------------------------------------------------------
// Methods.
// ------------------------------------------------------------------------

impl Signature {
    // Returns underlying byte array.
    pub fn as_slice(&self) -> &[Byte] {
        match self {
            Signature::ED25519(inner) => inner.as_slice(),
            Signature::SECP256K1(inner) => inner.as_slice(),
        }
    }

    // Returns underlying byte array prefixed with algorithm type tag.
    pub fn as_slice_with_tag(&self) -> Vec<u8> {
        let mut f = Vec::from(self.as_slice());
        f.insert(0, self.get_tag());

        f
    }

    // Returns algorithm type tag.
    pub fn get_tag(&self) -> Byte {
        match self {
            Signature::ED25519(_) => TAG_ED25519,
            Signature::SECP256K1(_) => TAG_SECP256K1,
        }
    }

    /// Verifies signature against arbitrary data.
    ///
    /// # Arguments
    ///
    /// * `vkey` - Verification key counterpart to signing key.
    /// * `data` - Data over which signature was issued.
    ///
    pub fn verify(&self, vkey: VerificationKey, data: &[Byte]) {
        match self {
            Signature::ED25519(sig) => match vkey {
                VerificationKey::ED25519(vk) => {
                    use ed25519_consensus::{Signature, VerificationKey};

                    let sig = Signature::try_from(sig.as_slice()).unwrap();
                    let vkey = VerificationKey::try_from(vk.as_slice()).unwrap();
                    assert_eq!(vkey.verify(&sig, &data), Ok(()));
                }
                _ => panic!("Invalid verification key type"),
            },
            Signature::SECP256K1(sig) => match vkey {
                VerificationKey::SECP256K1(vk) => {
                    use secp256k1::{ecdsa::Signature, Message, PublicKey, Secp256k1};

                    let msg = Message::from_digest_slice(data).unwrap();
                    let pbk = PublicKey::from_slice(vk.as_slice()).unwrap();
                    let sig = Signature::from_compact(&sig.as_slice()).unwrap();
                    assert_eq!(Secp256k1::new().verify_ecdsa(&msg, &sig, &pbk), Ok(()));
                }
                _ => panic!("Invalid verification key type"),
            },
        }
    }

    /// Verifies signature against a digest.
    ///
    /// # Arguments
    ///
    /// * `vkey` - Verification key counterpart to signing key.
    /// * `digest` - Digest over which signature was issued.
    ///
    pub fn verify_digest(&self, vkey: VerificationKey, digest: Digest) {
        self.verify(vkey, digest.as_slice());
    }
}

impl VerificationKey {
    // Returns underlying byte array.
    pub fn as_slice(&self) -> &[Byte] {
        match self {
            VerificationKey::ED25519(inner) => inner.as_slice(),
            VerificationKey::SECP256K1(inner) => inner.as_slice(),
        }
    }

    // Returns underlying byte array prefixed with algorithm type tag.
    pub fn as_slice_with_tag(&self) -> Vec<u8> {
        let mut f = Vec::from(self.as_slice());
        f.insert(0, self.get_tag());

        f
    }

    // Returns algorithm type tag.
    pub fn get_tag(&self) -> Byte {
        match self {
            VerificationKey::ED25519(_) => TAG_ED25519,
            VerificationKey::SECP256K1(_) => TAG_SECP256K1,
        }
    }
}

// ------------------------------------------------------------------------
// Traits.
// ------------------------------------------------------------------------

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Signature::ED25519(inner) => write!(f, "SIG:ED25519:{}", inner),
            Signature::SECP256K1(inner) => write!(f, "SIG:SECP256K1:{}", inner),
        }
    }
}

impl From<&str> for Signature {
    fn from(value: &str) -> Self {
        Self::from(hex::decode(value).unwrap())
    }
}

impl From<&[Byte]> for Signature {
    fn from(value: &[Byte]) -> Self {
        Self::new(&value)
    }
}

impl From<Vec<Byte>> for Signature {
    fn from(value: Vec<Byte>) -> Self {
        Self::from(value.as_slice())
    }
}

impl From<&Vec<Byte>> for Signature {
    fn from(value: &Vec<Byte>) -> Self {
        Self::from(value.as_slice())
    }
}

impl fmt::Display for VerificationKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VerificationKey::ED25519(inner) => write!(f, "VKEY:ED25519:{}", inner),
            VerificationKey::SECP256K1(inner) => write!(f, "VKEY:SECP256K1:{}", inner),
        }
    }
}

impl From<&str> for VerificationKey {
    fn from(value: &str) -> Self {
        Self::from(hex::decode(value).unwrap())
    }
}

impl From<&[Byte]> for VerificationKey {
    fn from(value: &[Byte]) -> Self {
        Self::new(&value)
    }
}

impl From<Vec<Byte>> for VerificationKey {
    fn from(value: Vec<Byte>) -> Self {
        Self::from(value.as_slice())
    }
}

impl From<&Vec<Byte>> for VerificationKey {
    fn from(value: &Vec<Byte>) -> Self {
        Self::from(value.as_slice())
    }
}

// ------------------------------------------------------------------------
// Traits -> serde.
// ------------------------------------------------------------------------

impl<'de> Deserialize<'de> for Signature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SignatureVistor;

        impl<'de> Visitor<'de> for SignatureVistor {
            type Value = Signature;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("supported formats: 64 char hex encoded string | 32 byte array")
            }

            fn visit_bytes<E>(self, v: &[Byte]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Signature::from(v))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Signature::from(v))
            }
        }

        deserializer.deserialize_any(SignatureVistor)
    }
}

impl Serialize for Signature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let as_hex = hex::encode(self.as_slice_with_tag());

        Ok(serializer.serialize_str(&as_hex).unwrap())
    }
}

impl<'de> Deserialize<'de> for VerificationKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct VerificationKeyVistor;

        impl<'de> Visitor<'de> for VerificationKeyVistor {
            type Value = VerificationKey;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("either a 64 char hex encoded string or a 32 byte array")
            }

            fn visit_bytes<E>(self, v: &[Byte]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(VerificationKey::from(v))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(VerificationKey::from(v))
            }
        }

        deserializer.deserialize_any(VerificationKeyVistor)
    }
}

impl Serialize for VerificationKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let as_hex = hex::encode(self.as_slice_with_tag());

        Ok(serializer.serialize_str(&as_hex).unwrap())
    }
}
