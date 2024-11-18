use super::digest::Digest;
use super::verification_key::VerificationKey;
use lutils::bites::Bytes64;
use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

// ------------------------------------------------------------------------
// Constants.
// ------------------------------------------------------------------------

const TAG_ED25519: u8 = 1;
const TAG_SECP256K1: u8 = 2;
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
    pub fn new(raw_bytes: &[u8]) -> Self {
        println!("dd {}", raw_bytes.len());
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
// Accessors.
// ------------------------------------------------------------------------

impl Signature {
    // Returns underlying byte array.
    pub fn as_slice(&self) -> &[u8] {
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
    pub fn get_tag(&self) -> u8 {
        match self {
            Signature::ED25519(_) => TAG_ED25519,
            Signature::SECP256K1(_) => TAG_SECP256K1,
        }
    }
}

// ------------------------------------------------------------------------
// Methods.
// ------------------------------------------------------------------------

impl Signature {
    /// Verifies signature against arbitrary data.
    ///
    /// # Arguments
    ///
    /// * `vkey` - Verification key counterpart to signing key.
    /// * `msg` - Data over which signature was issued.
    ///
    pub fn verify(&self, vkey: &VerificationKey, msg: &[u8]) {
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

impl From<&[u8]> for Signature {
    fn from(value: &[u8]) -> Self {
        Self::new(&value)
    }
}

impl From<Vec<u8>> for Signature {
    fn from(value: Vec<u8>) -> Self {
        Self::from(value.as_slice())
    }
}

impl From<&Vec<u8>> for Signature {
    fn from(value: &Vec<u8>) -> Self {
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

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
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

// ------------------------------------------------------------------------
// Tests.
// ------------------------------------------------------------------------

#[cfg(test)]
use rand::{rngs::OsRng, RngCore};

#[cfg(test)]
impl Signature {
    /// Returns a random `Signature`.
    #[cfg(any(feature = "testing", test))]
    pub fn random() -> Self {
        let mut random_bytes = [0u8; 64];
        OsRng.fill_bytes(&mut random_bytes);

        Self::new(random_bytes.as_slice())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex;

    const MSG: &str = "44682ea86b704fb3c65cd16f84a76b621e04bbdb3746280f25cf062220e471b4";
    const SIG_ED25519_TAGGED_HEX: &str = "012fa8e929a7514496545d098e86841463ef66358ff0930073fde3b138f66a2cef5304d884baa693a971d002d7e071f658fb16de8c1e5c80ba5ecea8b3866f8106";
    const SIG_SECP256K1_TAGGED_HEX: &str = "025ed6e5b71fa8f87dfb197a3d85c926d075f0b15651b59224a9a41a9fa1deb8cc2b2de5a8312a310af9b5321f67b744e1b3814994b13ec6db2769e9e6a9cc9364";
    const SIG_SET: [&str; 2] = [SIG_ED25519_TAGGED_HEX, SIG_SECP256K1_TAGGED_HEX];
    const VKEY_ED25519_TAGGED_HEX: &str =
        "01764f83295812c03354e0cd64718a7e50b452696799dc9d6e446338d668f3b2d9";
    const VKEY_SECP256K1_TAGGED_HEX: &str =
        "0203eed4eb0b40b3131679c365e3a23780eabfeaeb01776b0f908223ad1d4bd06f0d";
    const VKEY_SET: [&str; 2] = [VKEY_ED25519_TAGGED_HEX, VKEY_SECP256K1_TAGGED_HEX];

    const ED25519_SIG_TAGGED: &str = "012fa8e929a7514496545d098e86841463ef66358ff0930073fde3b138f66a2cef5304d884baa693a971d002d7e071f658fb16de8c1e5c80ba5ecea8b3866f8106";
    const ED25519_VKEY_TAGGED: &str =
        "01764f83295812c03354e0cd64718a7e50b452696799dc9d6e446338d668f3b2d9";
    const SECP256K1_VKEY_TAGGED: &str =
        "0203eed4eb0b40b3131679c365e3a23780eabfeaeb01776b0f908223ad1d4bd06f0d";
    const SECP256K1_SIG_TAGGED: &str = "025ed6e5b71fa8f87dfb197a3d85c926d075f0b15651b59224a9a41a9fa1deb8cc2b2de5a8312a310af9b5321f67b744e1b3814994b13ec6db2769e9e6a9cc9364";

    #[test]
    fn test_new_from_str() {
        for sig in SIG_SET {
            let _ = Signature::from(sig);
        }
    }

    #[test]
    fn test_new_from_vec() {
        for sig in SIG_SET {
            let as_vec = hex::decode(sig).unwrap();
            let _ = Signature::from(as_vec);
        }
    }

    #[test]
    fn test_destructure_of_tag() {
        let sig = Signature::from(ED25519_SIG_TAGGED);
        assert_eq!(sig.get_tag(), TAG_ED25519);

        let sig = Signature::from(SECP256K1_SIG_TAGGED);
        assert_eq!(sig.get_tag(), TAG_SECP256K1);
    }

    #[test]
    #[should_panic]
    fn test_panic_if_tag_is_invalid() {
        let vkey = format!("99{}", &ED25519_SIG_TAGGED[..ED25519_SIG_TAGGED.len() - 1]);
        let _ = Signature::from(vkey.as_str());
    }

    #[test]
    #[should_panic]
    fn test_panic_if_invalid_length_1() {
        let vkey = &ED25519_SIG_TAGGED[..ED25519_SIG_TAGGED.len() - 1];
        let _ = Signature::from(vkey);
    }

    #[test]
    #[should_panic]
    fn test_panic_if_invalid_length_2() {
        let vkey = format!("{}d9", ED25519_SIG_TAGGED);
        let _ = Signature::from(vkey.as_str());
    }

    // #[test]
    // #[should_panic]
    // fn test_panic_if_invalid() {
    //     let digest = Digest::random();

    //     digest.verify(MSG.to_vec());
    // }
}
