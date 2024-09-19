#![cfg_attr(target_os = "zkvm", no_main)]
#![cfg(target_os = "zkvm")]
sp1_zkvm::entrypoint!(main);

use std::panic;

use litmus_zk_lib::{
    constants, verify_digest, verify_signature, DigestBytes, VerificationKeyBytes,
};

/// Program entry point - wrapped by sp1 for execution within zk-vm.
///
/// Arguments are read from the zk-vm's i/o buffer:
///
/// byte 0..0 : Verification type.
/// byte 1..1 : Verification type.
pub fn main() {
    // Decode buffer 0..0.
    let typeof_verification = sp1_zkvm::io::read::<u8>();

    match typeof_verification {
        constants::VERIFICATION_TYPE_DIGEST => {
            // Decode buffer 1..1.
            let typeof_digest = sp1_zkvm::io::read::<u8>();

            // Decode buffer 2..34.
            let digest_bytes_raw = sp1_zkvm::io::read::<[u8; 32]>();

            // Decode buffer 35..N.
            let data = sp1_zkvm::io::read_vec();

            // Convert raw digest to typed digest.
            let digest_bytes = match typeof_digest {
                constants::DIGEST_TYPE_BLAKE2B => DigestBytes::BLAKE2B(digest_bytes_raw),
                _ => {
                    panic!("Unsupported digest type")
                }
            };

            // Verify.
            verify_digest(digest_bytes, data);
        }
        // byte 1..1 -> signature type.
        constants::VERIFICATION_TYPE_SIGNATURE => {
            // Decode buffer 1..1.
            let typeof_signature = sp1_zkvm::io::read::<u8>();

            // Decode buffer 2..33.
            let digest = sp1_zkvm::io::read::<[u8; 32]>();

            // Decode buffer 34..98.
            let signature: [u8; 64] = sp1_zkvm::io::read_vec().try_into().unwrap();

            // Decode buffer 100..[131|132].
            let verification_key = match typeof_signature {
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
            verify_signature(digest, signature, verification_key);
        }
        _ => {
            panic!("Unsupported verification type")
        }
    }
}
