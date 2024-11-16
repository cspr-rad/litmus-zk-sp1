use ltypes::chain::BlockWithProofs;

pub fn verify_block_with_proofs(entity: BlockWithProofs) {
    for proof in entity.proofs() {
        println!(
            "TODO verify block signature {} :: {} :: {}",
            entity.block().hash(),
            proof.signature(),
            proof.verification_key()
        )
    }

    // 1. Recompute block hash & assert equivalence.
    // 2. Verify signatures computed by validator public keys within era scope.
}
