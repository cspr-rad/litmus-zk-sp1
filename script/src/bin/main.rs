mod utils;

use clap::Parser;
use hex;
use sp1_sdk::{ProverClient, SP1Stdin};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const _ELF: &[u8] = include_bytes!("../../../elf/riscv32im-succinct-zkvm-elf");

/// The arguments for the command.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    execute: bool,

    #[clap(long)]
    prove: bool,
}

fn main() {
    // Set logging.
    sp1_sdk::utils::setup_logger();

    // Parse args.
    let args = Args::parse();
    if args.execute == args.prove {
        eprintln!("Error: You must specify either --execute or --prove");
        std::process::exit(1);
    }

    // Set inputs.
    // TODO: binary encode inputs ?
    let mut stdin = SP1Stdin::new();
    let fixtures = utils::get_fixtures();
    for fixture in fixtures.signatures {
        stdin.write_vec(hex::decode(fixture.data).unwrap());
        stdin.write_vec(hex::decode(fixture.sig).unwrap());
        stdin.write_vec(hex::decode(fixture.key.pbk).unwrap());
    }
    for fixture in fixtures.digests {
        stdin.write_vec(fixture.data.as_bytes().to_vec());
        stdin.write_vec(hex::decode(fixture.digest).unwrap());
    }

    // Execute | Prove.
    let client = ProverClient::new();
    if args.execute {
        let (output, report) = client.execute(_ELF, stdin).run().unwrap();
        println!("EXECUTION: # vm cycles   : {}", report.total_instruction_count());
        println!("EXECUTION: # calls to sys: {}", report.total_syscall_count());
    } else {
        let (pk, vk) = client.setup(_ELF);
        let proof = client
            .prove(&pk, stdin)
            .run()
            .expect("failed to generate proof");
        println!("PROOF: generation complete");
        client.verify(&proof, &vk).expect("failed to verify proof");
        println!("PROOF: verification complete");
    }
}
