use crate::fixtures::crypto as crypto_fixtures;
use crate::fixtures::crypto::{Digest as DigestFixture, Signature as SignatureFixture};
use crate::fixtures::Fixtures;
use ltypes::{
    chain::{BlockHeight, BlockWithProofs, EraId, ProtocolVersion},
    crypto::{Digest, PublicKey},
    misc::{SemanticVersion, Timestamp},
};
use lutils::bites::{Byte, Bytes32};
use serde_json::{Map, Value};
use sp1_sdk::SP1Stdin;
use std::collections::BTreeMap;

type BlockWithProofs1 = BlockWithProofs;

const VERIFICATION_TYPE_DIGEST: u8 = 0;
const VERIFICATION_TYPE_SIGNATURE: u8 = 1;
const VERIFICATION_TYPE_BLOCK_WITH_PROOFS: u8 = 10;
const DIGEST_TYPE_BLAKE2B: u8 = 0;
const ECC_TYPE_ED25519: u8 = 0;
const ECC_TYPE_SECP256K1: u8 = 1;

/// Convertor: Signature -> SignatureForVerification.
impl From<Fixtures> for Vec<SP1Stdin> {
    fn from(fixtures: Fixtures) -> Self {
        let mut result: Vec<SP1Stdin> = Vec::new();

        // Fixture set: cryptography.
        // for f in fixtures.crypto.digests {
        //     result.push(SP1Stdin::try_from(&f).unwrap());
        // }
        // for f in fixtures.crypto.signatures {
        //     result.push(SP1Stdin::try_from(&f).unwrap());
        // }

        result
            .push(SP1Stdin::try_from(&BlockWithProofsWrapper(fixtures.block_with_proofs)).unwrap());

        // Fixture set: chain.
        // TODO
        // let g = SP1Stdin::from(&fixtures.block_with_proofs);
        // result.push(get_stdin_from_block_with_proofs(
        //     &fixtures.block_with_proofs,
        // ));
        result
    }
}

impl From<&crypto_fixtures::Digest> for SP1Stdin {
    fn from(value: &crypto_fixtures::Digest) -> Self {
        let mut zk_stdin = Self::new();
        zk_stdin.write(&VERIFICATION_TYPE_DIGEST);
        match value.algo.as_str() {
            "BLAKE2B" => {
                zk_stdin.write(&DIGEST_TYPE_BLAKE2B);
            }
            _ => {
                panic!("Unsupported hashing algo.")
            }
        }
        zk_stdin.write_vec(hex::decode(&value.digest).unwrap());
        zk_stdin.write_vec(value.data.as_bytes().to_vec());

        zk_stdin
    }
}

impl From<&crypto_fixtures::Signature> for SP1Stdin {
    fn from(value: &crypto_fixtures::Signature) -> Self {
        let mut zk_stdin = Self::new();
        zk_stdin.write(&VERIFICATION_TYPE_SIGNATURE);
        match value.key.algo.as_str() {
            "ED25519" => {
                zk_stdin.write(&ECC_TYPE_ED25519);
            }
            "SECP256K1" => {
                zk_stdin.write(&ECC_TYPE_SECP256K1);
            }
            _ => {
                panic!("Unsupported signature algo.")
            }
        }
        zk_stdin.write_vec(hex::decode(&value.data).unwrap());
        zk_stdin.write_vec(hex::decode(&value.sig).unwrap());
        zk_stdin.write_vec(hex::decode(&value.key.pbk).unwrap());

        zk_stdin
    }
}

struct BlockWithProofsWrapper(BlockWithProofs);

impl BlockWithProofsWrapper {
    fn inner(&self) -> &BlockWithProofs {
        &self.0
    }
}

impl From<&BlockWithProofsWrapper> for SP1Stdin {
    fn from(wrapper: &BlockWithProofsWrapper) -> Self {
        let mut zk_stdin = Self::new();
        zk_stdin.write(&VERIFICATION_TYPE_BLOCK_WITH_PROOFS);
        zk_stdin.write_vec(serde_cbor::to_vec(&wrapper.inner()).unwrap());

        zk_stdin
    }
}
