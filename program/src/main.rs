#![cfg_attr(target_os = "zkvm", no_main)]
#![cfg(target_os = "zkvm")]
sp1_zkvm::entrypoint!(main);

use lutils::bites::Byte;

mod chain;
mod constants;
mod crypto;

/// Program entry point - wrapped by sp1 for execution within zk-vm.
///
/// N.B. Arguments are parsed from SP1 ZK-VM i/o buffer:
pub fn main() {
    let verification_type_tag = sp1_zkvm::io::read::<Byte>();
    match verification_type_tag {
        constants::VERIFICATION_TYPE_BLOCK_WITH_PROOFS => {
            chain::verify_block_with_proofs(sp1_zkvm::io::read_vec())
        }
        constants::VERIFICATION_TYPE_DIGEST => {
            crypto::verify_digest(sp1_zkvm::io::read_vec(), sp1_zkvm::io::read_vec())
        }
        constants::VERIFICATION_TYPE_DIGEST_SIGNATURE => {
            crypto::verify_digest_signature(
                sp1_zkvm::io::read_vec(),
                sp1_zkvm::io::read_vec(),
                sp1_zkvm::io::read_vec(),
            );
        }
        _ => {
            panic!("Unsupported verification type")
        }
    }
}
