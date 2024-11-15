use serde::{Deserialize, Serialize};

// Digest information required for verification.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Digest {
    // Hashing algorithm used to compute digest.
    pub algo: String,

    // Hex representation of data over which digest was computed.
    pub data: String,

    // Computed digest.
    #[serde(with = "hex::serde")]
    pub digest: Vec<u8>,
}

// ECC key pair information.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct KeyPair {
    // ECC algorithm used to compute digest.
    pub algo: String,

    // Binary representation of ECC verification key.
    #[serde(with = "hex::serde")]
    pub pbk: Vec<u8>,

    // Binary representation of ECC signing key.
    #[serde(with = "hex::serde")]
    pub pvk: Vec<u8>,
}

// Signature information.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Signature {
    // Hex representation of data over which signature was computed.
    #[serde(with = "hex::serde")]
    pub data: Vec<u8>,

    // ECC key pair used to compute / verify signature.
    pub key: KeyPair,

    // Binary representation of computed signature.
    #[serde(with = "hex::serde")]
    pub sig: Vec<u8>,
}

// Mapped signature information required for verification.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SignatureForVerification {
    // Binary representation of data over which signature was computed.
    #[serde(with = "hex::serde")]
    pub msg: Vec<u8>,

    // Binary representation of ECC verification key.
    #[serde(with = "hex::serde")]
    pub pbk: Vec<u8>,

    // Binary representation of computed signature.
    #[serde(with = "hex::serde")]
    pub sig: Vec<u8>,
}

// Set of fixtures to be verified within a zk-vm.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Fixtures {
    // Set of digests for verification.
    pub digests: Vec<Digest>,

    // Set of signatures for verification.
    pub signatures: Vec<Signature>,
}

/// Convertor: Signature -> SignatureForVerification.
impl From<Signature> for SignatureForVerification {
    fn from(value: Signature) -> Self {
        Self {
            msg: value.data,
            pbk: value.key.pbk,
            sig: value.sig,
        }
    }
}
