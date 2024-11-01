use std::panic;

use crate::constants;
use litmus_zk_lib::{
    Byte, Bytes32, Bytes64, Digest, DigestBytes, SignatureBytesRaw, VerificationKeyBytes,
};

/// Verifies a digest over a byte vector.
pub fn verify_digest() {
    // Parse input byte stream.
    // 0     : digest type tag.
    // 1..33 : digest bytes.
    // 34..N : data over which digest has been computed.
    fn parse_input_stream() -> (Byte, Bytes32, Vec<Byte>) {
        (
            sp1_zkvm::io::read::<u8>(),
            sp1_zkvm::io::read::<Bytes32>(),
            sp1_zkvm::io::read_vec(),
        )
    }

    // Set inputs.
    let (digest_type_tag, digest_bytes, data) = parse_input_stream();

    // Map raw digest -> typed digest.
    let digest = match digest_type_tag {
        constants::DIGEST_TYPE_BLAKE2B => Digest::new_blake2b(digest_bytes),
        _ => {
            panic!("Unsupported digest type")
        }
    };

    // Invoke verification function.
    digest.verify(data);
}

/// Verifies a digest signature.
pub fn verify_signature() {
    // Parse input byte stream.
    // 0      : signature type tag.
    // 1..64  : signature bytes.
    // 65..96 : digest over which signature has been computed.
    // 97..N  : verification key.
    fn parse_input_stream() -> (Byte, Bytes64, Bytes32, Vec<Byte>) {
        (
            sp1_zkvm::io::read::<Byte>(),
            // sp1_zkvm::io::read::<Bytes64>(),
            sp1_zkvm::io::read_vec().try_into().unwrap(),
            sp1_zkvm::io::read::<Bytes32>(),
            sp1_zkvm::io::read_vec(),
        )
    }

    // Set inputs.
    let (signature_type_tag, signature, digest, verification_key_raw) = parse_input_stream();

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

    // Invoke verification function.
    verification_key.verify_signature_over_digest(signature, DigestBytes::new(digest));
}
