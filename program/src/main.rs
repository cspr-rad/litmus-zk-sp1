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
    // Parse input byte stream.
    // 0      : verification type tag.
    fn parse_input_stream() -> Byte {
        sp1_zkvm::io::read::<Byte>()
    }

    // Set inputs.
    let verification_type_tag = parse_input_stream();

    // Invoke verification function.
    match verification_type_tag {
        constants::VERIFICATION_TYPE_BLOCK_WITH_PROOFS => chain::verify_block_with_proofs(),
        constants::VERIFICATION_TYPE_DIGEST => {
            let (encoded_digest, msg) = (sp1_zkvm::io::read_vec(), sp1_zkvm::io::read_vec());

            crypto::verify_digest(encoded_digest, msg)
        }
        constants::VERIFICATION_TYPE_DIGEST_SIGNATURE => {
            let (encoded_digest, encoded_sig, encoded_vkey) = (
                sp1_zkvm::io::read_vec(),
                sp1_zkvm::io::read_vec(),
                sp1_zkvm::io::read_vec(),
            );

            crypto::verify_digest_signature(encoded_digest, encoded_sig, encoded_vkey)
        }
        _ => {
            panic!("Unsupported verification type")
        }
    }
}
