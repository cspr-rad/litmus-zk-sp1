use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod blocks;
pub mod crypto;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Fixtures {
    pub block: Value,
    pub crypto: crypto::Fixtures,
}
