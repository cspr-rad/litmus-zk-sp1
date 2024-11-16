use super::constants;
use crate::fixtures::{
    wrapped::{WrappedBlockWithProofs, WrappedDigest, WrappedSignature},
    Fixtures,
};
use sp1_sdk::SP1Stdin;

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

impl From<&WrappedDigest> for SP1Stdin {
    fn from(value: &WrappedDigest) -> Self {
        let mut vm_stdin = Self::new();
        vm_stdin.write(&constants::VERIFICATION_TYPE_DIGEST);
        vm_stdin.write_vec(serde_cbor::to_vec(value.inner()).unwrap());
        vm_stdin.write_vec(value.data());

        vm_stdin
    }
}

impl From<&WrappedSignature> for SP1Stdin {
    fn from(value: &WrappedSignature) -> Self {
        let mut vm_stdin = Self::new();
        vm_stdin.write(&constants::VERIFICATION_TYPE_SIGNATURE);
        vm_stdin.write_vec(serde_cbor::to_vec(value.inner()).unwrap());
        vm_stdin.write_vec(value.data());
        vm_stdin.write_vec(value.vkey());

        vm_stdin
    }
}

impl From<&WrappedBlockWithProofs> for SP1Stdin {
    fn from(value: &WrappedBlockWithProofs) -> Self {
        let mut vm_stdin = Self::new();
        vm_stdin.write(&constants::VERIFICATION_TYPE_BLOCK_WITH_PROOFS);
        vm_stdin.write_vec(serde_cbor::to_vec(value.inner()).unwrap());

        vm_stdin
    }
}
