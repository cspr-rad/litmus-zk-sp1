use ltypes::{
    chain::{
        Block, BlockHash, BlockHeight, BlockV1, BlockV1Body, BlockV1Header, BlockV2, BlockV2Body,
        BlockV2Header, EraId, ProtocolVersion,
    },
    crypto::{Digest, PublicKey},
    misc::{SemanticVersion, Timestamp},
};
use lutils::bites::Bytes32;
use serde_json::{Map, Value};

use crate::fixtures::blocks::Block as JsonBlock;

pub(super) fn get_chain_block_from_json(value: &Value) -> Block {
    fn get_hash(value: &str) -> BlockHash {
        BlockHash::new(Digest::BLAKE2B(Bytes32::from(value)))
    }

    fn get_v1(value: &Map<String, Value>) -> BlockV1 {
        fn get_body(_: &Value) -> BlockV1Body {
            unimplemented!("get_body")
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
}

pub(super) fn get_block_from_json(value: &str) -> JsonBlock {
    serde_json::from_str(value).unwrap()
}
