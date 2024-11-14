use crate::fixtures::crypto::{Digest as DigestFixture, Signature as SignatureFixture};
use crate::fixtures::Fixtures;
use ltypes::{
    chain::{BlockHeight, BlockWithProofs, EraId, ProtocolVersion},
    crypto::{Digest, PublicKey},
    misc::{SemanticVersion, Timestamp},
};
use lutils::bites::{Byte, Bytes32};
use serde_json::{Map, Value};
use sp1_sdk::SP1Stdin;
use std::collections::BTreeMap;

const VERIFICATION_TYPE_DIGEST: u8 = 0;
const VERIFICATION_TYPE_SIGNATURE: u8 = 1;
const DIGEST_TYPE_BLAKE2B: u8 = 0;
const ECC_TYPE_ED25519: u8 = 0;
const ECC_TYPE_SECP256K1: u8 = 1;

/// Convertor: Signature -> SignatureForVerification.
impl From<Fixtures> for Vec<SP1Stdin> {
    fn from(fixtures: Fixtures) -> Self {
        let mut result = Vec::new();

        // Fixture set: cryptography.
        // for fixture in fixtures.crypto.digests {
        //     result.push(get_stdin_for_crypto_digest(&fixture));
        // }
        // for fixture in fixtures.crypto.signatures {
        //     result.push(get_stdin_for_crypto_signature(&fixture));
        // }

        // Fixture set: chain.
        // TODO
        result.push(get_stdin_from_block_with_proofs(
            &fixtures.block_with_proofs,
        ));

        panic!("DDD");
    }
}

fn get_stdin_from_block_with_proofs(block_with_proofs: &BlockWithProofs) -> SP1Stdin {
    println!("{:?}", block_with_proofs);
    unimplemented!("get_stdin_for_chain_block")
}

fn get_stdin_for_crypto_digest(value: &DigestFixture) -> SP1Stdin {
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

fn get_stdin_for_crypto_signature(value: &SignatureFixture) -> SP1Stdin {
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

// header = get_header(&value["block"]["Version2"]);

// TODO: map raw JSON object to typed doain object
// Block::new_v2(BlockV2::new(
//     body: BlockV2Body::new(Vec::new(), BTreeMap::new()),
//     header: BlockV2Header::new(
//         Digest::new_blake2b(
//             Bytes32::from(value["block"]["Version2"]["header"]["accumulated_seed"].to_string()).to_vec()
//         ),
//         Digest::new_blake2b(
//             Bytes32::from(value["block"]["Version2"]["header"]["body_hash"].to_string()).to_vec()
//         ),
//         Digest::new_blake2b(
//             u8::from(value["block"]["Version2"]["header"]["current_gas_price"].to_string())
//         ),
//         current_gas_price,
//         era_end, era_id,
//         height,
//         last_switch_block_hash,
//         parent_hash,
//         proposer,
//         protocol_version,
//         random_bit,
//         state_root_hash,
//         timestamp
//     )
// ))
