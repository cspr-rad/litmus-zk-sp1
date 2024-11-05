use serde::{Deserialize, Serialize};
use sp1_sdk::SP1Stdin;

const VERIFICATION_TYPE_DIGEST: u8 = 0;
const VERIFICATION_TYPE_SIGNATURE: u8 = 1;
const DIGEST_TYPE_BLAKE2B: u8 = 0;
const ECC_TYPE_ED25519: u8 = 0;
const ECC_TYPE_SECP256K1: u8 = 1;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Digest {
    pub data: String,
    pub algo: String,
    pub digest: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct KeyPair {
    pub algo: String,
    pub pbk: String,
    pub pvk: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Signature {
    pub data: String,
    pub key: KeyPair,
    pub sig: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SignatureForVerification {
    pub msg: String,
    pub pbk: String,
    pub sig: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
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

/// Convertor: Signature -> SignatureForVerification.
impl From<Fixtures> for Vec<SP1Stdin> {
    fn from(value: Fixtures) -> Self {
        let mut result = Vec::new();
        for fixture in value.digests {
            result.push(get_stdin_for_verify_digest(fixture));
        }
        for fixture in value.signatures {
            result.push(get_stdin_for_verify_signature(fixture));
        }

        result
    }
}

fn get_stdin_for_verify_digest(fixture: Digest) -> SP1Stdin {
    let mut stdin = SP1Stdin::new();
    stdin.write(&VERIFICATION_TYPE_DIGEST);
    match fixture.algo.as_str() {
        "BLAKE2B" => {
            stdin.write(&DIGEST_TYPE_BLAKE2B);
        }
        _ => {
            panic!("Unsupported hashing algo.")
        }
    }
    stdin.write_vec(hex::decode(fixture.digest).unwrap());
    stdin.write_vec(fixture.data.as_bytes().to_vec());

    stdin
}

fn get_stdin_for_verify_signature(fixture: Signature) -> SP1Stdin {
    let mut stdin = SP1Stdin::new();
    stdin.write(&VERIFICATION_TYPE_SIGNATURE);
    match fixture.key.algo.as_str() {
        "ED25519" => {
            stdin.write(&ECC_TYPE_ED25519);
        }
        "SECP256K1" => {
            stdin.write(&ECC_TYPE_SECP256K1);
        }
        _ => {
            panic!("Unsupported signature algo.")
        }
    }
    stdin.write_vec(hex::decode(fixture.data).unwrap());
    stdin.write_vec(hex::decode(fixture.sig).unwrap());
    stdin.write_vec(hex::decode(fixture.key.pbk).unwrap());

    stdin
}
