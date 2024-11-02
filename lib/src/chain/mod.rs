use std::panic;

mod types;

use types::{BlockHash, BlockSignature, EraConsensusInfo, EraId};

/// 1. Compute digest over era_id + block_hash.
/// 2. Iterate block signatures.
///     2.1. Verify sig is member of era_validator_weights.
///     2.2. Verify sig.
///     2.3. Tally cumulative weight.
///     2.4. If cumulative weight > consensus threshold -> TRUE | exit
/// 3. Panic if cumulative weight < consensus threshold
pub fn verify_block(
    block_hash: BlockHash,
    block_signatures: Vec<BlockSignature>,
    era_consensus_info: EraConsensusInfo,
) {
    /// Compute digest over which signature was issued.
    // era_consensus_info.era_id()
    /// Validate raw signatures.
    panic!("TODO: verify a block");
}

/// Verifies a single block signature.
pub fn verify_block_signature(
    block_hash: BlockHash,
    block_signature: BlockSignature,
    era_id: EraId,
) {
    /// Step 1. Recompute digest over block_hash + era_id.
    /// Step 2. Verify signature.
    panic!("TODO: verify a block signature");
}

pub fn compute_bytes_to_sign(block_hash: &BlockHash, era_id: EraId) -> Vec<u8> {
    panic!("TODO: compute block hash");

    // let mut bytes = block_hash.inner().to_vec();
    // bytes.extend_from_slice(&era_id.inner().to_le_bytes());
    // bytes
}
