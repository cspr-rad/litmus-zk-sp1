use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Block {
    pub body: String,
    pub hash: String,
    pub header: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BlockBody {
    pub rewarded_signatures: Vec<String>,
    pub transactions: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BlockHeader {
    pub accumulated_seed: String,
    pub body_hash: String,
    pub current_gas_price: String,
    pub era_id: u64,
    pub height: u64,
    pub last_switch_block_hash: String,
    pub parent_hash: String,
    pub proposer: String,
    pub protocol_version: String,
    pub random_bit: bool,
    pub state_root_hash: String,
    pub timestamp: String,
}
