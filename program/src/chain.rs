use ltypes::chain::BlockWithProofs;

pub fn verify_block_with_proofs(encoded: Vec<u8>) {
    let entity: BlockWithProofs = serde_cbor::from_slice(&encoded).unwrap();

    println!("TODO: verify a block with proofs: {:?}", entity);

    // 1. Recompute block hash & assert equivalence.
    // 2. Verify signatures computed by validator public keys within era scope.
}
