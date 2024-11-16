use serde::{Deserialize, Serialize};

pub mod crypto;
pub mod wrapped;

use ltypes::chain::BlockHash;
use wrapped::{WrappedBlockWithProofs, WrappedDigest, WrappedSignature};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Fixtures {
    pub set_of_blocks_with_proofs: Vec<WrappedBlockWithProofs>,
    pub set_of_digests: Vec<WrappedDigest>,
    pub set_of_signatures: Vec<WrappedSignature>,
    pub trusted_block_hash: BlockHash,
}
