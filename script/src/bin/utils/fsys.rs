use crate::fixtures::{
    crypto::{Digest as DigestFixture, Fixtures as CryptoFixtures, Signature as SignatureFixture},
    wrapped::{WrappedBlockV2WithProofs, WrappedDigest, WrappedSignature},
    Fixtures,
};
use ltypes::{
    chain::{BlockHash, BlockWithProofs, ChainNameDigest},
    crypto::{Digest, Signature, VerificationKey},
};
use std::fs;

// TODO: scan folder and derive.
const BLOCK_RANGE_MIN: u32 = 469;
const BLOCK_RANGE_MAX: u32 = 474;

pub fn get_fixtures() -> Fixtures {
    let crypto_fixtures = get_crypto_fixtures();
    let chain_name_digest = get_chain_name_digest();

    Fixtures {
        set_of_blocks_with_proofs: get_set_of_blocks_with_proofs(&chain_name_digest),
        set_of_digests: get_set_of_digests(&crypto_fixtures.digests),
        set_of_signatures: get_set_of_signatures(&crypto_fixtures.signatures),
        trusted_block_hash: get_trusted_block_hash(),
    }
}

fn get_chain_name_digest() -> ChainNameDigest {
    let chain_name = get_fixture_content(String::from("network_name.txt"));

    ChainNameDigest::from(chain_name.trim())
}

fn get_crypto_fixtures() -> CryptoFixtures {
    serde_json::from_str(&get_fixture_content(String::from("crypto.json"))).unwrap()
}

fn get_fixture_content(fname: String) -> String {
    let path = get_path_to_fixture(fname);

    fs::read_to_string(path).unwrap()
}

fn get_path_to_fixture(fname: String) -> String {
    format!("fixtures/{fname}")
}

fn get_set_of_blocks_with_proofs(
    chain_name_digest: &ChainNameDigest,
) -> Vec<WrappedBlockV2WithProofs> {
    fn get_inner(block_id: u32) -> BlockWithProofs {
        let fname = format!("block-{block_id}.json");

        serde_json::from_str(&get_fixture_content(fname)).unwrap()
    }

    fn get_one(block_id: u32, chain_name_digest: &ChainNameDigest) -> WrappedBlockV2WithProofs {
        WrappedBlockV2WithProofs(get_inner(block_id), chain_name_digest.to_owned())
    }

    (BLOCK_RANGE_MIN..BLOCK_RANGE_MAX)
        .map(|x| get_one(x, chain_name_digest))
        .collect()
}

fn get_set_of_digests(f_set: &Vec<DigestFixture>) -> Vec<WrappedDigest> {
    fn get_one(f: &DigestFixture) -> WrappedDigest {
        WrappedDigest(
            Digest::from(&f.digest),
            f.msg.as_bytes().to_vec().to_owned(),
        )
    }

    f_set.iter().map(get_one).collect()
}

fn get_set_of_signatures(f_set: &Vec<SignatureFixture>) -> Vec<WrappedSignature> {
    fn get_one(f: &SignatureFixture) -> WrappedSignature {
        WrappedSignature(
            Signature::from(&f.tagged_sig),
            VerificationKey::from(&f.tagged_pbk),
            Digest::from(&f.msg),
        )
    }

    f_set.iter().map(get_one).collect()
}

fn get_trusted_block_hash() -> BlockHash {
    let as_hex = get_fixture_content(String::from("block_hash_trusted.txt"));

    BlockHash::new(Digest::from(as_hex.trim()))
}
