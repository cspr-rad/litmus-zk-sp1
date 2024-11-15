use serde::{Deserialize, Serialize};

pub mod chain;
pub mod crypto;

use chain::WrappedBlockWithProofs;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Fixtures {
    pub set_of_blocks_with_proofs: Vec<WrappedBlockWithProofs>,
    pub crypto: crypto::Fixtures,
}
