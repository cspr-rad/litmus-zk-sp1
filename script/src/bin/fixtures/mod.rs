use serde::{Deserialize, Serialize};

pub mod blocks;
pub mod crypto;

pub use blocks::Block as JsonBlock;
pub use blocks::BlockWithProofs as JsonBlockWithProofs;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Fixtures {
    pub block_with_proofs: JsonBlockWithProofs,
    pub crypto: crypto::Fixtures,
}
