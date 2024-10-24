#![cfg_attr(target_os = "zkvm", no_main)]
#![cfg(target_os = "zkvm")]
sp1_zkvm::entrypoint!(main);

mod chain;
mod constants;
mod crypto;

use std::panic;

/// Program entry point - wrapped by sp1 for execution within zk-vm.
///
/// N.B. Arguments are read from the zk-vm's i/o buffer:
pub fn main() {
    // Buffer 0..0: verification type tag.
    let verification_type_tag = sp1_zkvm::io::read::<u8>();
    match verification_type_tag {
        constants::VERIFICATION_TYPE_BLOCK => chain::do_block_verification(),
        constants::VERIFICATION_TYPE_DIGEST => crypto::do_digest_verification(),
        constants::VERIFICATION_TYPE_SIGNATURE => crypto::do_signature_verification(),
        _ => {
            panic!("Unsupported verification type")
        }
    }
}
