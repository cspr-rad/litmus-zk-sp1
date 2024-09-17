use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Digest {
    pub data: String,
    pub algo: String,
    pub digest: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyPair {
    pub algo: String,
    pub pbk: String,
    pub pvk: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Signature {
    pub data: String,
    pub key: KeyPair,
    pub sig: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignatureForVerification {
    pub msg: String,
    pub pbk: String,
    pub sig: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Fixtures {
    pub digests: Vec<Digest>,
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
