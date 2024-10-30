#![cfg_attr(target_os = "zkvm", no_main)]
#![cfg(target_os = "zkvm")]
sp1_zkvm::entrypoint!(main);

mod chain;
mod constants;
mod crypto;
use litmus_zk_lib::Byte;

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
        constants::VERIFICATION_TYPE_BLOCK => chain::verify_block(),
        constants::VERIFICATION_TYPE_DIGEST => crypto::verify_digest(),
        constants::VERIFICATION_TYPE_SIGNATURE => crypto::verify_signature(),
        _ => {
            panic!("Unsupported verification type")
        }
    }
}
