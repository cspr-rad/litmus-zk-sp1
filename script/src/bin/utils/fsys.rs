use crate::fixtures::{
    crypto::{Digest as DigestFixture, Fixtures as CryptoFixtures, Signature as SignatureFixture},
    wrapped::{WrappedBlockWithProofs, WrappedDigest, WrappedSignature},
    Fixtures,
};
use lcrypto::{Digest, Signature};
use std::{collections::btree_map::Range, fs};

// TODO: scan folder and derive.
const BLOCK_RANGE_MIN: u32 = 469;
const BLOCK_RANGE_MAX: u32 = 474;

pub fn get_fixtures() -> Fixtures {
    fn get_set_of_blocks_with_proofs() -> Vec<WrappedBlockWithProofs> {
        fn get_one(block_id: u32) -> WrappedBlockWithProofs {
            let fname = format!("block-{block_id}");

            serde_json::from_str(&get_fixture_content(fname)).unwrap()
        }

        (BLOCK_RANGE_MIN..BLOCK_RANGE_MAX).map(get_one).collect()
    }

    fn get_set_of_digests(fixtures: &Vec<DigestFixture>) -> Vec<WrappedDigest> {
        fn get_one(f: &DigestFixture) -> WrappedDigest {
            WrappedDigest(Digest::from(&f.digest))
        }

        fixtures.iter().map(get_one).collect()
    }

    fn get_set_of_signatures(fixtures: &Vec<SignatureFixture>) -> Vec<WrappedSignature> {
        fn get_one(f: &SignatureFixture) -> WrappedSignature {
            WrappedSignature(Signature::from(&f.tagged_sig))
        }

        fixtures.iter().map(get_one).collect()
    }

    fn get_crypto_fixtures() -> CryptoFixtures {
        serde_json::from_str(&get_fixture_content(String::from("crypto"))).unwrap()
    }

    let crypto_fixtures = get_crypto_fixtures();

    Fixtures {
        set_of_blocks_with_proofs: get_set_of_blocks_with_proofs(),
        set_of_digests: get_set_of_digests(&crypto_fixtures.digests),
        set_of_signatures: get_set_of_signatures(&crypto_fixtures.signatures),
        crypto: crypto_fixtures,
    }
}

fn get_fixture_content(typeof_fixture: String) -> String {
    let path = get_fixture_path(typeof_fixture);

    fs::read_to_string(path).unwrap()
}

fn get_fixture_path(typeof_fixture: String) -> String {
    format!("fixtures/{typeof_fixture}.json")
}
