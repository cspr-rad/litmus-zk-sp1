use std::panic;

use crate::constants;
use litmus_zk_lib::{Byte, Bytes, Bytes32, Bytes64, Digest, Signature, VerificationKey};

/// Verifies a digest over a byte vector.
pub fn verify_digest() {
    // Parse input byte stream.
    // 0     : digest type tag.
    // 1..33 : digest bytes.
    // 34..N : data over which digest has been computed.
    fn parse_input_stream() -> (Byte, Bytes32, Vec<Byte>) {
        (
            sp1_zkvm::io::read(),
            sp1_zkvm::io::read_vec().try_into().unwrap(),
            sp1_zkvm::io::read_vec(),
        )
    }

    // Set inputs.
    let (digest_type_tag, digest_bytes, data) = parse_input_stream();

    // Map raw digest -> typed digest.
    let digest = match digest_type_tag {
        constants::DIGEST_TYPE_BLAKE2B => Digest::BLAKE2B(digest_bytes),
        _ => {
            panic!("Unsupported digest type")
        }
    };

    // Verify.
    digest.verify(data);
}

/// Verifies a signature over a digest.
pub fn verify_digest_signature() {
    // Parse input byte stream.
    // 0      : signature type tag.
    // 1..32 : digest over which signature has been computed.
    // 33..96  : signature bytes.
    // 97..N  : verification key.
    fn parse_input_stream() -> (Byte, Bytes32, Bytes64, Vec<Byte>) {
        (
            sp1_zkvm::io::read(),
            sp1_zkvm::io::read_vec().try_into().unwrap(),
            sp1_zkvm::io::read_vec().try_into().unwrap(),
            sp1_zkvm::io::read_vec(),
        )
    }

    // Set inputs.
    let (signature_type_tag, digest, signature, verification_key) = parse_input_stream();

    // Map raw keys -> typed keys.
    let (signature, verification_key) = match signature_type_tag {
        constants::SIGNATURE_TYPE_ED25519 => (
            Signature::ED25519(signature),
            VerificationKey::ED25519(verification_key.try_into().unwrap()),
        ),
        constants::SIGNATURE_TYPE_SECP256K1 => (
            Signature::SECP256K1(signature),
            VerificationKey::SECP256K1(verification_key.try_into().unwrap()),
        ),
        _ => {
            panic!("Unsupported signature type")
        }
    };

    // Verify.
    signature.verify(verification_key, digest.as_slice());
}
