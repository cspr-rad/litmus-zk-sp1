use crate::fixtures::crypto::{Digest, Signature};
use crate::fixtures::Fixtures;
use ltypes::chain::Block;
use serde_json::Value;
use sp1_sdk::SP1Stdin;

const VERIFICATION_TYPE_DIGEST: u8 = 0;
const VERIFICATION_TYPE_SIGNATURE: u8 = 1;
const DIGEST_TYPE_BLAKE2B: u8 = 0;
const ECC_TYPE_ED25519: u8 = 0;
const ECC_TYPE_SECP256K1: u8 = 1;

/// Convertor: Signature -> SignatureForVerification.
impl From<Fixtures> for Vec<SP1Stdin> {
    fn from(value: Fixtures) -> Self {
        let mut result = Vec::new();

        // Fixture set: cryptography.
        // for fixture in value.crypto.digests {
        //     result.push(get_stdin_for_crypto_digest(&fixture));
        // }
        // for fixture in value.crypto.signatures {
        //     result.push(get_stdin_for_crypto_signature(&fixture));
        // }

        // Fixture set: chain.
        // TODO
        result.push(get_stdin_for_chain_block(&value.block));

        result
    }
}

fn get_stdin_for_chain_block(value: &Value) -> SP1Stdin {
    // TODO: map raw JSON object to typed doain object
    println!("{:?}", value["block"]["Version2"]["hash"]);

    unimplemented!()
}

fn get_stdin_for_crypto_digest(value: &Digest) -> SP1Stdin {
    let mut stdin = SP1Stdin::new();
    stdin.write(&VERIFICATION_TYPE_DIGEST);
    match value.algo.as_str() {
        "BLAKE2B" => {
            stdin.write(&DIGEST_TYPE_BLAKE2B);
        }
        _ => {
            panic!("Unsupported hashing algo.")
        }
    }
    stdin.write_vec(hex::decode(&value.digest).unwrap());
    stdin.write_vec(value.data.as_bytes().to_vec());

    stdin
}

fn get_stdin_for_crypto_signature(value: &Signature) -> SP1Stdin {
    let mut stdin = SP1Stdin::new();
    stdin.write(&VERIFICATION_TYPE_SIGNATURE);
    match value.key.algo.as_str() {
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
    stdin.write_vec(hex::decode(&value.data).unwrap());
    stdin.write_vec(hex::decode(&value.sig).unwrap());
    stdin.write_vec(hex::decode(&value.key.pbk).unwrap());

    stdin
}
