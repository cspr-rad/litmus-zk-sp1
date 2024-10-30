use std::panic;

use crate::constants;
use litmus_zk_lib::{
    Bytes32, Bytes64, Digest, DigestBytes, SignatureBytesRaw, VerificationKeyBytes,
};

pub fn verify_digest() {
    fn get_inputs() -> (u8, Bytes32, Vec<u8>) {
        // 0     : digest type tag.
        // 1..33 : digest bytes.
        // 34..N : data over which digest has been computed.
        (
            sp1_zkvm::io::read::<u8>(),
            sp1_zkvm::io::read::<Bytes32>(),
            sp1_zkvm::io::read_vec(),
        )
    }

    // Decode inputs.
    let (digest_type_tag, digest_bytes, data) = get_inputs();

    // Map raw digest -> typed digest.
    let digest = match digest_type_tag {
        constants::DIGEST_TYPE_BLAKE2B => Digest::BLAKE2B(DigestBytes::new(digest_bytes)),
        _ => {
            panic!("Unsupported digest type")
        }
    };

    // Verify.
    digest.verify(data);
}

pub fn verify_signature() {
    fn get_inputs() -> (u8, Bytes64, Bytes32, Vec<u8>) {
        // 0      : signature type tag.
        // 1..64  : signature bytes.
        // 65..96 : digest over which signature has been computed.
        // 97..N  : verification key.
        (
            sp1_zkvm::io::read::<u8>(),
            sp1_zkvm::io::read_vec().try_into().unwrap(),
            sp1_zkvm::io::read::<Bytes32>(),
            sp1_zkvm::io::read_vec(),
        )
    }

    // Decode inputs.
    let (signature_type_tag, signature, digest, verification_key_raw) = get_inputs();

    // Map raw verification key -> typed verification key.
    let verification_key = match signature_type_tag {
        constants::SIGNATURE_TYPE_ED25519 => {
            VerificationKeyBytes::ED25519(verification_key_raw.try_into().unwrap())
        }
        constants::SIGNATURE_TYPE_SECP256K1 => {
            VerificationKeyBytes::SECP256K1(verification_key_raw.try_into().unwrap())
        }
        _ => {
            panic!("Unsupported signature type")
        }
    };

    // Verify.
    verification_key.verify_signature_over_digest(signature, DigestBytes::new(digest));
}
