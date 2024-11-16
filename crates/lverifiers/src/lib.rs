use ltypes::chain::BlockWithProofs;

pub fn verify_block_with_proofs(entity: BlockWithProofs) {
    // 1. Validate signatures.
    // TODO: optimise ... validate sigs in weight order
    let msg = entity.block().get_bytes_for_finality_signature();
    println!("bytes_for_finality_signature :: {:?}", msg);

    for proof in entity.proofs() {
        proof
            .signature()
            .verify(proof.verification_key(), msg.as_slice());
    }

    // 1. Recompute block hash & assert equivalence.
    // 2. Verify signatures computed by validator public keys within era scope.
}
