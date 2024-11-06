use serde::{Deserialize, Serialize};

// Digest information required for verification.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Digest {
    // Hashing algorithm used to compute digest.
    pub algo: String,

    // Hex representation of data over which digest was computed.
    pub data: String,

    // Computed digest.
    pub digest: String,
}

// ECC key pair information.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct KeyPair {
    // ECC algorithm used to compute digest.
    pub algo: String,

    // Hex representation of ECC verification key.
    pub pbk: String,

    // Hex representation of ECC signing key.
    pub pvk: String,
}

// Signature information.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Signature {
    // Hex representation of data over which signature was computed.
    pub data: String,

    // ECC key pair used to compute / verify signature.
    pub key: KeyPair,

    // Hex representation of computed signature.
    pub sig: String,
}

// Mapped signature information required for verification.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SignatureForVerification {
    // Hex representation of data over which signature was computed.
    pub msg: String,

    // Hex representation of ECC verification key.
    pub pbk: String,

    // Hex representation of computed signature.
    pub sig: String,
}

// Set of fixtures to be verified within a zk-vm.
#[derive(Serialize, Deserialize, Clone, Debug)]
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
