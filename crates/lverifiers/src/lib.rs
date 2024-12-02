mod digests;

use ltypes::chain::{Block, BlockHash, BlockWithProofs, ChainNameDigest, EraConsensusInfo};

pub fn verify_block_v1_with_proofs(_: BlockWithProofs) {
    unimplemented!("verify_block_v1_with_proofs");
}

/// Verifies a version two block.
///
/// # Arguments
///
/// * `entity` - Block to be verified.
/// * `chain_name_digest` - Digest over name of a blockchain.
/// * `era_consensus_info` - Information pulled from a previous era necessary to tally finality signatures.
///
pub fn verify_block_v2_with_proofs(
    block_with_proofs: BlockWithProofs,
    chain_name_digest: ChainNameDigest,
    era_consensus_info: Option<EraConsensusInfo>,
) {
    // Destructure inner block.
    let block = match block_with_proofs.block() {
        Block::V2(inner) => inner,
        _ => panic!("Invalid block version."),
    };

    // Error if invalid recomputed block hash.
    // assert_eq!(block.hash(), &BlockHash::from(block.get_bytes_for_hash()));

    // Error if invalid finality signature.
    let msg = block.get_bytes_for_finality_signature(&chain_name_digest);
    for proof in block_with_proofs.proofs() {
        proof.signature().verify(proof.verification_key(), &msg);
    }
}

// def verify_block(
//     block: Block,
//     switch_block_of_previous_era: Block = None,
//     ) -> Block:
//     """Validates a block.

//     :block: A block to be validated.
//     :switch_block_of_previous_era: The switch block of previous consensus era.
//     :returns: The block if considered valid, otherwise raises exception.

//     """
//     # TODO: extend input parameters -> blockID: if hash then ina  descent, if height then ascending & switch block must be available

//     # BL-000: Exception if block was not downloaded.
//     if block is None:
//         raise InvalidBlockException(InvalidBlockExceptionType.NotFound)

//     # BL-001: Exception if recomputed block hash is not equal to actual block hash.
//     # TODO: implement digest computation
//     if block.hash != factory.create_digest_of_block(block.header):
//         pass
//         # raise InvalidBlockException(InvalidBlockExceptionType.InvalidHash)

//     # BL-002: Exception if switch block is not from a previous era.
//     if switch_block_of_previous_era is not None:
//         if switch_block_of_previous_era.header.era_id != block.header.era_id - 1:
//             raise InvalidBlockException(InvalidBlockExceptionType.InvalidEra)

//     # BL-003: Exception if a block signatory is not an era signatory.
//     if switch_block_of_previous_era is not None:
//         for signatory in block.signatories:
//             if signatory not in switch_block_of_previous_era.header.era_end.next_era_signatories:
//                 raise InvalidBlockException(InvalidBlockExceptionType.InvalidProposer)

//     # BL-004: Exception if a finality signature is invalid.
//     block_digest_for_finality_signature: bytes = \
//         factory.create_digest_of_block_for_finality_signature(block)
//     for proof in block.proofs:
//         if not crypto.is_signature_valid(
//             block_digest_for_finality_signature,
//             proof.signature.algo,
//             proof.signature.sig,
//             proof.public_key.pbk,
//         ):
//             raise InvalidBlockException(InvalidBlockExceptionType.InvalidFinalitySignature)

//     # BL-005: Exception if weight of finality signatures is insufficient.
//     if switch_block_of_previous_era is not None:
//         proven_weight: Weight = \
//             block.get_finality_signature_weight(switch_block_of_previous_era)
//         required_weight: Weight = \
//             switch_block_of_previous_era.validator_weight_required_for_finality_in_next_era
//         if proven_weight < required_weight:
//             raise InvalidBlockException(InvalidBlockExceptionType.InsufficientFinalitySignatureWeight)

//     return block
