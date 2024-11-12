// use alloc::collections::BTreeMap;
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Block {
    #[serde(rename = "Version2")]
    V2(BlockV2),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BlockProof {
    #[serde(with = "hex::serde")]
    public_key: Vec<u8>,
    #[serde(with = "hex::serde")]
    signature: [u8; 65],
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BlockV2 {
    // pub body: BlockV2Body,
    hash: String,
    header: BlockV2Header,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BlockV2Body {
    // pub rewarded_signatures: Vec<String>,
    // pub transactions: BTreeMap<u8, Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BlockV2Header {
    #[serde(with = "hex::serde")]
    accumulated_seed: [u8; 32],
    #[serde(with = "hex::serde")]
    body_hash: [u8; 32],
    current_gas_price: u8,
    era_id: u64,
    height: u64,
    #[serde(with = "hex::serde")]
    last_switch_block_hash: [u8; 32],
    #[serde(with = "hex::serde")]
    parent_hash: [u8; 32],
    #[serde(with = "hex::serde")]
    proposer: Vec<u8>,
    protocol_version: String,
    random_bit: bool,
    #[serde(with = "hex::serde")]
    state_root_hash: [u8; 32],
    timestamp: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BlockWithProofs {
    block: Block,
    proofs: Vec<BlockProof>,
}

// ------------------------------------------------------------------------
// Accessors.
// ------------------------------------------------------------------------

impl BlockV2 {
    pub fn body(&self) -> BlockV2Body {
        unimplemented!()
    }
    pub fn hash(&self) -> &String {
        &self.hash
    }
    pub fn header(&self) -> &BlockV2Header {
        &self.header
    }
}

impl BlockWithProofs {
    pub fn block(&self) -> &Block {
        &self.block
    }
    pub fn proofs(&self) -> &Vec<BlockProof> {
        &self.proofs
    }
}
