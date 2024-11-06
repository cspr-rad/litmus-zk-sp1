use super::types::{BlockHash, BlockSignature, EraConsensusInfo, EraId};
use crate::crypto::Digest;

/// 1. Compute digest over era_id + block_hash.
/// 2. Iterate block signatures.
///     2.1. Verify sig is member of era_validator_weights.
///     2.2. Verify sig.
///     2.3. Tally cumulative weight.
///     2.4. If cumulative weight > consensus threshold -> TRUE | exit
/// 3. Panic if cumulative weight < consensus threshold
pub fn verify_block(
    block_hash: &BlockHash,
    finality_signatures: &Vec<BlockSignature>,
    era_consensus_info: EraConsensusInfo,
) {
    // Set digest over which finality signatures were issued.
    let digest_for_finality_signature =
        compute_digest_for_finality_signature(block_hash, era_consensus_info.era_id());

    // Verify finality signatures.
    for finality_signature in finality_signatures {
        verify_block_signature(&digest_for_finality_signature, finality_signature);
    }

    panic!("TODO: verify a block");
}

/// Verifies a single block signature.
pub fn verify_block_signature(
    digest_of_finality_signature: &Digest,
    block_signature: &BlockSignature,
) {
    panic!(
        "TODO: verify block signature: {:?}: {:?}",
        digest_of_finality_signature, block_signature
    );
}

fn compute_digest_for_finality_signature(block_hash: &BlockHash, era_id: &EraId) -> Digest {
    panic!(
        "TODO: compute_digest_of_finality_signature: {:?}: {:?}",
        block_hash, era_id
    );
}
