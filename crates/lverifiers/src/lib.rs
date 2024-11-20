use ltypes::chain::Block;
use ltypes::chain::BlockWithProofs;
use ltypes::chain::ChainNameDigest;

pub fn verify_block_v1_with_proofs(_: BlockWithProofs) {
    unimplemented!("verify_block_v1_with_proofs");
}

pub fn verify_block_v2_with_proofs(entity: BlockWithProofs, chain_name_digest: ChainNameDigest) {
    // 1. Validate finality signatures.
    let msg = match entity.block() {
        Block::V1(_) => panic!(
            "Block type version to verifier mismatch.  Invoke verify_block_v1_with_proofs instead."
        ),
        Block::V2(inner) => inner.get_bytes_for_finality_signature(&chain_name_digest),
    };
    for proof in entity.proofs() {
        proof.signature().verify(proof.verification_key(), &msg);
    }

    // 1. Recompute block hash & assert equivalence.
    // 2. Verify signatures computed by validator public keys within era scope.
}
