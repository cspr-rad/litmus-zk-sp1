use super::digest::Digest;
use hex;
use lutils::bites::{Byte, Bytes32, Bytes33, Bytes64};

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Signature {
    ED25519(Bytes64),
    SECP256K1(Bytes64),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum VerificationKey {
    ED25519(Bytes32),
    SECP256K1(Bytes33),
}

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl Signature {
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
// Accessors.
// ------------------------------------------------------------------------

impl Signature {
    // Returns underlying byte array.
    pub fn as_slice(&self) -> &[Byte] {
        match self {
            Signature::ED25519(inner) => inner.as_slice(),
            Signature::SECP256K1(inner) => inner.as_slice(),
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
    /// * `data` - Data over which signature was issued.
    ///
    pub fn verify(&self, vkey: VerificationKey, data: &[Byte]) {
        match self {
            Signature::ED25519(sig) => match vkey {
                VerificationKey::ED25519(vk) => verify_ed25519(sig, vk, data),
                _ => panic!("Invalid verification key type"),
            },
            Signature::SECP256K1(sig) => match vkey {
                VerificationKey::SECP256K1(vk) => verify_sec256k1(sig, vk, data),
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

/// Verifies ED25519 ECC signature.
fn verify_ed25519(sig: &Bytes64, vkey: Bytes32, data: &[Byte]) {
    use ed25519_consensus::{Signature, VerificationKey};

    let sig = Signature::try_from(sig.as_slice()).unwrap();
    let vkey = VerificationKey::try_from(vkey.as_slice()).unwrap();

    assert_eq!(vkey.verify(&sig, &data), Ok(()));
}

/// Verifies SECP256K1 ECC signature.
fn verify_sec256k1(sig: &Bytes64, vkey: Bytes33, data: &[Byte]) {
    use secp256k1::{ecdsa::Signature, Message, PublicKey, Secp256k1};

    let msg = Message::from_digest_slice(data).unwrap();
    let pbk = PublicKey::from_slice(vkey.as_slice()).unwrap();
    let sig = Signature::from_compact(&sig.as_slice()).unwrap();

    assert_eq!(Secp256k1::new().verify_ecdsa(&msg, &sig, &pbk), Ok(()));
}

// ------------------------------------------------------------------------
// Traits.
// ------------------------------------------------------------------------

// From -> borrowed string slice -> self.
impl From<&str> for VerificationKey {
    fn from(value: &str) -> Self {
        let raw_bytes = hex::decode(value).unwrap();
        match raw_bytes[0] {
            1_u8 => VerificationKey::ED25519(Bytes32::from(raw_bytes[1..].to_vec())),
            2_u8 => VerificationKey::SECP256K1(Bytes33::from(raw_bytes[1..].to_vec())),
            _ => panic!("Unsupported verification key type prefix"),
        }
    }
}
