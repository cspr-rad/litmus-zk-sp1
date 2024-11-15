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

// Signature information.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Signature {
    // ECC algorithm used to compute digest.
    pub algo: String,

    // Hex representation of data over which signature was computed.
    #[serde(with = "hex::serde")]
    pub data: Vec<u8>,

    // Binary representation of ECC verification key.
    #[serde(with = "hex::serde")]
    pub pbk: Vec<u8>,

    // Binary representation of ECC signing key.
    #[serde(with = "hex::serde")]
    pub pvk: Vec<u8>,

    // Binary representation of computed signature.
    #[serde(with = "hex::serde")]
    pub sig: Vec<u8>,

    // Binary representation of computed signature with algo type prefix.
    #[serde(with = "hex::serde")]
    pub tagged_sig: Vec<u8>,
}

// Set of fixtures to be verified within a zk-vm.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Fixtures {
    // Set of digests for verification.
    pub digests: Vec<Digest>,

    // Set of signatures for verification.
    pub signatures: Vec<Signature>,
}
