use ltypes::chain::BlockWithProofs;
use lverifiers;

pub fn verify_block_with_proofs(encoded: Vec<u8>) {
    let entity: BlockWithProofs = serde_cbor::from_slice(&encoded).unwrap();

    lverifiers::verify_block_with_proofs(entity);
}
