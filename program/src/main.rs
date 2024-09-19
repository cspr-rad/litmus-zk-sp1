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
    // byte 0..0 -> verification type.
    match sp1_zkvm::io::read::<u8>() {
        // byte 1..1 -> digest type.
        constants::VERIFICATION_TYPE_DIGEST => match sp1_zkvm::io::read::<u8>() {
            // byte 2..34 -> digest.
            // byte 35..N -> data.
            constants::DIGEST_TYPE_BLAKE2B => {
                verify_digest(
                    DigestBytes::BLAKE2B(sp1_zkvm::io::read_vec().try_into().unwrap()),
                    sp1_zkvm::io::read_vec(),
                );
            }
            _ => {
                panic!("Unsupported digest type")
            }
        },
        // byte 1..1 -> signature type.
        constants::VERIFICATION_TYPE_SIGNATURE => match sp1_zkvm::io::read::<u8>() {
            constants::SIGNATURE_TYPE_ED25519 => {
                verify_signature(
                    sp1_zkvm::io::read_vec().try_into().unwrap(),
                    sp1_zkvm::io::read_vec().try_into().unwrap(),
                    VerificationKeyBytes::ED25519(sp1_zkvm::io::read_vec().try_into().unwrap()),
                );
            }
            constants::SIGNATURE_TYPE_SECP256K1 => {
                verify_signature(
                    sp1_zkvm::io::read_vec().try_into().unwrap(),
                    sp1_zkvm::io::read_vec().try_into().unwrap(),
                    VerificationKeyBytes::SECP256K1(sp1_zkvm::io::read_vec().try_into().unwrap()),
                );
            }
            _ => {
                panic!("Unsupported signature type")
            }
        },
        _ => {
            panic!("Unsupported verification type")
        }
    }
}
