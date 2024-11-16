use ltypes::chain::BlockWithProofs;

pub fn verify_block_with_proofs(encoded: Vec<u8>) {
    let entity: BlockWithProofs = serde_cbor::from_slice(&encoded).unwrap();

    println!("TODO: verifiy a block with proofs");
}
