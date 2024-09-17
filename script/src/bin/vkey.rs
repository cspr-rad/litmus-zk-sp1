//! A script to print the program verification key.
//!
//! You can run this script using the following command:
//! ```shell
//! RUST_LOG=info cargo run --bin vkey --release
//! ```

use sp1_sdk::{HashableKey, ProverClient};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const _ELF: &[u8] = include_bytes!("../../../elf/riscv32im-succinct-zkvm-elf");

fn main() {
    // Setup: logger.
    sp1_sdk::utils::setup_logger();

    // Setup: prover client.
    let client = ProverClient::new();

    // Setup: program.
    let (_, vk) = client.setup(_ELF);

    // Print verification key.
    println!("Program Verification Key: {}", vk.bytes32());
}
