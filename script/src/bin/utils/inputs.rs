use super::constants;
use crate::fixtures::{
    crypto::{Digest, Signature},
    wrapped::{WrappedBlockWithProofs, WrappedDigest, WrappedSignature},
    Fixtures,
};
use sp1_sdk::SP1Stdin;

/// Convertor: Signature -> SignatureForVerification.
impl From<Fixtures> for Vec<SP1Stdin> {
    fn from(fixtures: Fixtures) -> Self {
        let mut result: Vec<SP1Stdin> = Vec::new();

        for f in fixtures.set_of_digests {
            result.push(SP1Stdin::try_from(&f).unwrap());
        }
        // for f in fixtures.set_of_signatures {
        //     result.push(SP1Stdin::try_from(&f).unwrap());
        // }
        // for f in fixtures.set_of_blocks_with_proofs {
        //     result.push(SP1Stdin::try_from(&f).unwrap());
        // }

        result
    }
}

impl From<&Digest> for SP1Stdin {
    fn from(value: &Digest) -> Self {
        let mut zk_stdin = Self::new();
        zk_stdin.write(&constants::VERIFICATION_TYPE_DIGEST);
        match value.algo.as_str() {
            "BLAKE2B" => {
                zk_stdin.write(&constants::DIGEST_TYPE_BLAKE2B);
            }
            _ => {
                panic!("Unsupported hashing algo.")
            }
        }
        zk_stdin.write_vec(value.digest.to_owned());
        zk_stdin.write_vec(value.data.as_bytes().to_vec());

        zk_stdin
    }
}

fn from_digest(digest: &WrappedDigest, data: &Vec<u8>) -> SP1Stdin {
    let mut zk_stdin = SP1Stdin::new();
    zk_stdin.write(&constants::VERIFICATION_TYPE_DIGEST);
    zk_stdin.write_vec(serde_cbor::to_vec(&digest.inner()).unwrap());
    zk_stdin.write_vec(data.to_owned());

    zk_stdin
}

impl From<&WrappedDigest> for SP1Stdin {
    fn from(value: &WrappedDigest) -> Self {
        let mut zk_stdin = Self::new();
        zk_stdin.write(&constants::VERIFICATION_TYPE_DIGEST);
        zk_stdin.write_vec(serde_cbor::to_vec(&value.inner()).unwrap());

        zk_stdin
    }
}

impl From<&Signature> for SP1Stdin {
    fn from(value: &Signature) -> Self {
        let mut zk_stdin = Self::new();
        zk_stdin.write(&constants::VERIFICATION_TYPE_SIGNATURE);
        match value.algo.as_str() {
            "ED25519" => {
                zk_stdin.write(&constants::ECC_TYPE_ED25519);
            }
            "SECP256K1" => {
                zk_stdin.write(&constants::ECC_TYPE_SECP256K1);
            }
            _ => {
                panic!("Unsupported signature algo.")
            }
        }
        zk_stdin.write_vec(value.data.to_owned());
        zk_stdin.write_vec(value.sig.to_owned());
        zk_stdin.write_vec(value.pbk.to_owned());

        zk_stdin
    }
}

impl From<&WrappedSignature> for SP1Stdin {
    fn from(value: &WrappedSignature) -> Self {
        let mut zk_stdin = Self::new();
        zk_stdin.write(&constants::VERIFICATION_TYPE_SIGNATURE);
        zk_stdin.write_vec(serde_cbor::to_vec(&value.inner()).unwrap());

        zk_stdin
    }
}

impl From<&WrappedBlockWithProofs> for SP1Stdin {
    fn from(value: &WrappedBlockWithProofs) -> Self {
        let mut zk_stdin = Self::new();
        zk_stdin.write(&constants::VERIFICATION_TYPE_BLOCK_WITH_PROOFS);
        zk_stdin.write_vec(serde_cbor::to_vec(&value.inner()).unwrap());

        zk_stdin
    }
}
