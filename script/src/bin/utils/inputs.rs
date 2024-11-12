use crate::fixtures::crypto::{Digest as DigestFixture, Signature as SignatureFixture};
use crate::fixtures::Fixtures;
use ltypes::{
    chain::{BlockHeight, EraId, ProtocolVersion},
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
        // let mut result = Vec::new();

        // Fixture set: cryptography.
        // for fixture in fixtures.crypto.digests {
        //     result.push(get_stdin_for_crypto_digest(&fixture));
        // }
        // for fixture in fixtures.crypto.signatures {
        //     result.push(get_stdin_for_crypto_signature(&fixture));
        // }

        // Fixture set: chain.
        // TODO
        println!("{:?}", fixtures.block_with_proofs);
        // result.push(get_stdin_for_chain_block(&fixtures.block));

        panic!("DDD");
    }
}

fn get_stdin_for_chain_block(value: &Value) -> SP1Stdin {
    use ltypes::chain::{
        Block, BlockHash, BlockV1, BlockV1Body, BlockV1Header, BlockV2, BlockV2Body, BlockV2Header,
    };

    fn get_block(value: &Value) -> Block {
        fn get_v1(value: &Map<String, Value>) -> BlockV1 {
            fn get_body(_: &Value) -> BlockV1Body {
                unimplemented!("get_body")
            }

            fn get_hash(value: &str) -> BlockHash {
                BlockHash::new(Digest::BLAKE2B(Bytes32::from(value)))
            }

            fn get_header(_: &Map<String, Value>) -> BlockV1Header {
                unimplemented!("get_header")
            }

            BlockV1::new(
                get_body(&value["body"]),
                get_hash(value["hash"].as_str().unwrap()),
                get_header(value["header"].as_object().unwrap()),
            )
        }

        fn get_v2(value: &Map<String, Value>) -> BlockV2 {
            fn get_body(_: &Value) -> Option<BlockV2Body> {
                Option::None
            }

            fn get_hash(value: &str) -> BlockHash {
                BlockHash::new(Digest::BLAKE2B(Bytes32::from(value)))
            }

            fn get_header(value: &Map<String, Value>) -> BlockV2Header {
                BlockV2Header::new(
                    Digest::BLAKE2B(Bytes32::from(value["accumulated_seed"].as_str().unwrap())),
                    Digest::BLAKE2B(Bytes32::from(value["body_hash"].as_str().unwrap())),
                    u8::try_from(value["current_gas_price"].as_u64().unwrap()).unwrap(),
                    Option::None,
                    EraId::new(value["era_id"].as_u64().unwrap()),
                    BlockHeight::new(value["height"].as_u64().unwrap()),
                    Option::Some(BlockHash::new(Digest::BLAKE2B(Bytes32::from(
                        value["last_switch_block_hash"].as_str().unwrap(),
                    )))),
                    BlockHash::new(Digest::BLAKE2B(Bytes32::from(
                        value["parent_hash"].as_str().unwrap(),
                    ))),
                    PublicKey::from(value["proposer"].as_str().unwrap()),
                    ProtocolVersion::new(SemanticVersion::new(2, 0, 0)),
                    value["random_bit"].as_bool().unwrap(),
                    Digest::BLAKE2B(Bytes32::from(value["state_root_hash"].as_str().unwrap())),
                    Timestamp::new(value["timestamp"].as_number().unwrap().as_u128().unwrap()),
                )
            }

            BlockV2::new(
                get_body(&value["body"]),
                get_hash(value["hash"].as_str().unwrap()),
                get_header(value["header"].as_object().unwrap()),
            )
        }

        match value.get("block") {
            Option::Some(value) => match value.get("Version1") {
                Option::Some(_) => unimplemented!("Version1"),
                Option::None => match value.get("Version2") {
                    Option::Some(inner) => Block::new_v2(get_v2(inner.as_object().unwrap())),
                    Option::None => panic!("Unsupported block version"),
                },
            },
            Option::None => panic!("Invalid JSON"),
        }

        // match value.get("Version1").unwrap() {
        //     Value::Object(kv_map) => Block::new_v1(get_v1(kv_map)),
        //     Value::Null => match value.get("Version2").unwrap() {
        //         Value::Object(kv_map) => Block::new_v2(get_v2(kv_map)),
        //         Value::Null => panic!("Unsupported block version."),
        //         _ => panic!("Invalid block JSON."),
        //     },
        //     _ => panic!("Invalid block JSON."),
        // }
    }

    fn get_stdin(block: Block) -> SP1Stdin {
        unimplemented!()
    }

    get_stdin(get_block(&value))
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
