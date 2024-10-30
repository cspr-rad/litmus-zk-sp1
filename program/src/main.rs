#![cfg_attr(target_os = "zkvm", no_main)]
#![cfg(target_os = "zkvm")]
sp1_zkvm::entrypoint!(main);

mod chain;
mod constants;
mod crypto;

/// Program entry point - wrapped by sp1 for execution within zk-vm.
///
/// N.B. Arguments are parsed from SP1 ZK-VM i/o buffer:
pub fn main() {
    fn parse_input_stream() -> u8 {
        // 0      : verification type tag.
        sp1_zkvm::io::read::<u8>()
    }

    // Parse input byte stream.
    let verification_type_tag = parse_input_stream();

    // Invoke verification function.
    match verification_type_tag {
        constants::VERIFICATION_TYPE_BLOCK => chain::verify_block(),
        constants::VERIFICATION_TYPE_DIGEST => crypto::verify_digest(),
        constants::VERIFICATION_TYPE_SIGNATURE => crypto::verify_signature(),
        _ => {
            panic!("Unsupported verification type")
        }
    }
}
