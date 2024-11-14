use serde::{Deserialize, Serialize};

pub mod blocks;
pub mod crypto;

use ltypes::chain::BlockWithProofs;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Fixtures {
    pub block_with_proofs: BlockWithProofs,
    pub crypto: crypto::Fixtures,
}
