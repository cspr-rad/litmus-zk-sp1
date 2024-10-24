use std::panic;

use crate::constants;
use litmus_zk_lib::{DigestBytes, DigestBytesRaw, SignatureBytesRaw, VerificationKeyBytes};

pub fn do_digest_verification() {
    // Buffer 1..1: digest type tag.
    let digest_type_tag = sp1_zkvm::io::read::<u8>();

    // Buffer 2..34: digest bytes.
    let digest_bytes_raw = sp1_zkvm::io::read::<DigestBytesRaw>();

    // Buffer 35..N: data over which digest has been computed.
    let data = sp1_zkvm::io::read_vec();

    // Map raw digest -> typed digest.
    let digest_bytes = match digest_type_tag {
        constants::DIGEST_TYPE_BLAKE2B => DigestBytes::BLAKE2B(digest_bytes_raw),
        _ => {
            panic!("Unsupported digest type")
        }
    };

    // Verify.
    digest_bytes.verify(data);
}

pub fn do_signature_verification() {
    // Buffer 1..1: signature type tag.
    let signature_type_tag = sp1_zkvm::io::read::<u8>();

    // Buffer 2..65: signature bytes.
    let signature: SignatureBytesRaw = sp1_zkvm::io::read_vec().try_into().unwrap();

    // Buffer 66..98: digest bytes.
    let digest = sp1_zkvm::io::read::<[u8; 32]>();

    // Buffer 100..[131|132]: verification key.
    let verification_key = match signature_type_tag {
        constants::SIGNATURE_TYPE_ED25519 => {
            VerificationKeyBytes::ED25519(sp1_zkvm::io::read_vec().try_into().unwrap())
        }
        constants::SIGNATURE_TYPE_SECP256K1 => {
            VerificationKeyBytes::SECP256K1(sp1_zkvm::io::read_vec().try_into().unwrap())
        }
        _ => {
            panic!("Unsupported signature type")
        }
    };

    // Verify.
    verification_key.verify_signature_over_digest(signature, digest);
}
