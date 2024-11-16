use super::digest::Digest;
use super::verification_key::VerificationKey;
use lutils::bites::{Byte, Bytes64};
use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

// ------------------------------------------------------------------------
// Constants.
// ------------------------------------------------------------------------

const TAG_ED25519: Byte = 1;
const TAG_SECP256K1: Byte = 2;
const SIG_SIZE: usize = 64;

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Signature {
    ED25519(Bytes64),
    SECP256K1(Bytes64),
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
    /// * `msg` - Data over which signature was issued.
    ///
    pub fn verify(&self, vkey: &VerificationKey, msg: &[Byte]) {
        match self {
            Signature::ED25519(sig) => match vkey {
                VerificationKey::ED25519(vk) => {
                    use ed25519_consensus::{Signature, VerificationKey};

                    let sig = Signature::try_from(sig.as_slice()).unwrap();
                    let vkey = VerificationKey::try_from(vk.as_slice()).unwrap();
                    match vkey.verify(&sig, &msg) {
                        Result::Ok(_) => {}
                        Result::Err(_) => panic!("ED25519 signature verification failure"),
                    }
                }
                _ => panic!("Invalid verification key type"),
            },
            Signature::SECP256K1(sig) => match vkey {
                VerificationKey::SECP256K1(vk) => {
                    use secp256k1::{ecdsa::Signature, Message, PublicKey, Secp256k1};

                    let msg = Message::from_digest_slice(msg).unwrap();
                    let pbk = PublicKey::from_slice(vk.as_slice()).unwrap();
                    let sig = Signature::from_compact(&sig.as_slice()).unwrap();
                    match Secp256k1::new().verify_ecdsa(&msg, &sig, &pbk) {
                        Result::Ok(_) => {}
                        Result::Err(_) => panic!("SECP256K1 signature verification failure"),
                    }
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
    pub fn verify_digest(&self, vkey: &VerificationKey, digest: &Digest) {
        self.verify(&vkey, &digest.as_slice());
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
