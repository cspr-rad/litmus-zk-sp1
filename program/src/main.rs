#![no_main]
sp1_zkvm::entrypoint!(main);

use litmus_zk_lib::{verify_digest, verify_signature, DigestBytes, VerificationKeyBytes};

/// Program entry point - wrapped by sp1 for execution within zk-vm.
///
/// Arguments are read from the zk-vm's i/o buffer.
pub fn main() {
    // ED25519 signature verification.
    verify_signature(
        sp1_zkvm::io::read_vec().try_into().unwrap(),
        sp1_zkvm::io::read_vec().try_into().unwrap(),
        VerificationKeyBytes::ED25519(sp1_zkvm::io::read_vec().try_into().unwrap()),
    );

    // ED25519 signature verification.
    verify_signature(
        sp1_zkvm::io::read_vec().try_into().unwrap(),
        sp1_zkvm::io::read_vec().try_into().unwrap(),
        VerificationKeyBytes::SECP256K1(sp1_zkvm::io::read_vec().try_into().unwrap()),
    );

    // BLAKE2B signature verification.
    verify_digest(
        sp1_zkvm::io::read_vec(),
        DigestBytes::BLAKE2B(sp1_zkvm::io::read_vec().try_into().unwrap()),
    );
}
