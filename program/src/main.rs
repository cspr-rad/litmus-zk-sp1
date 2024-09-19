#![cfg_attr(target_os = "zkvm", no_main)]
#![cfg(target_os = "zkvm")]
sp1_zkvm::entrypoint!(main);

use std::panic;

use litmus_zk_lib::{
    constants, verify_digest, verify_signature, DigestBytes, VerificationKeyBytes,
};

/// Program entry point - wrapped by sp1 for execution within zk-vm.
///
/// Arguments are read from the zk-vm's i/o buffer.
pub fn main() {
    match sp1_zkvm::io::read::<u8>() {
        constants::VERIFICATION_TYPE_DIGEST => match sp1_zkvm::io::read::<u8>() {
            constants::DIGEST_TYPE_BLAKE2B => {
                verify_digest(
                    sp1_zkvm::io::read_vec(),
                    DigestBytes::BLAKE2B(sp1_zkvm::io::read_vec().try_into().unwrap()),
                );
            }
            _ => {
                panic!("Unsupported digest type")
            }
        },
        constants::VERIFICATION_TYPE_SIGNATURE => match sp1_zkvm::io::read::<u8>() {
            constants::SIGNATURE_TYPE_ED25519 => {
                println!("SIG: {:?} :: {:?}", 1, 0);
            }
            constants::SIGNATURE_TYPE_SECP256K1 => {
                println!("SIG: {:?} :: {:?}", 1, 1);
            }
            _ => {
                panic!("Unsupported signature type")
            }
        },
        _ => {
            panic!("Unsupported verification type")
        }
    }

    // if verification_type == 0 {
    //     let verification_subtype = sp1_zkvm::io::read::<u8>();

    //     println!("SIG: {:?} :: {:?}", verification_type, verification_subtype);
    // } else if verification_type == 1 {
    //     let verification_subtype = sp1_zkvm::io::read::<u8>();
    //     println!("DIG: {:?} :: {:?}", verification_type, verification_subtype);
    //     // // BLAKE2B signature verification.
    //     // verify_digest(
    //     //     sp1_zkvm::io::read_vec(),
    //     //     DigestBytes::BLAKE2B(sp1_zkvm::io::read_vec().try_into().unwrap()),
    //     // );
    // }

    // // ED25519 signature verification.
    // verify_signature(
    //     sp1_zkvm::io::read_vec().try_into().unwrap(),
    //     sp1_zkvm::io::read_vec().try_into().unwrap(),
    //     VerificationKeyBytes::ED25519(sp1_zkvm::io::read_vec().try_into().unwrap()),
    // );

    // // ED25519 signature verification.
    // verify_signature(
    //     sp1_zkvm::io::read_vec().try_into().unwrap(),
    //     sp1_zkvm::io::read_vec().try_into().unwrap(),
    //     VerificationKeyBytes::SECP256K1(sp1_zkvm::io::read_vec().try_into().unwrap()),
    // );
}
