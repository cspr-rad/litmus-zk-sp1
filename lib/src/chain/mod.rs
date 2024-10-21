use std::panic;

mod types;

use types::{BlockHash, EraConsensusInfo};

/// 1. Compute digest over era_id + block_hash.
/// 2. Iterate block signatures.
///     2.1. Verify sig is member of era_validator_weights.
///     2.2. Verify sig.
///     2.3. Tally cumulative weight.
///     2.4. If cumulative weight > consensus threshold -> TRUE | exit
/// 3. Panic if cumulative weight < consensus threshold
pub fn verify_block(
    block_hash: BlockHash,
    block_signatures: u64,
    era_consensus_info: EraConsensusInfo,
) {
    panic!("TODO: verify a block");
}
