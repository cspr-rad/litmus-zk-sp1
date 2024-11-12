mod fixtures;
mod utils;

use clap::Parser;
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

    // Set args.
    let args = Args::parse();
    if args.execute == args.prove {
        eprintln!("Error: You must specify either --execute or --prove");
        std::process::exit(1);
    }

    use crate::fixtures::JsonBlock;
    let fixtures = utils::get_fixtures();
    match fixtures.block_with_proofs.block() {
        JsonBlock::V2(ref block) => {
            println!("{:?}", block.hash());
        }
        _ => panic!("Invalid block info"),
    }

    // Iterate stdin set & invoke.
    for stdin in Vec::<SP1Stdin>::from(fixtures) {
        if args.execute {
            do_pgm_execute(&args, &stdin);
        } else {
            do_pgm_prove(&args, &stdin);
        }
    }
}

fn do_pgm_execute(args: &Args, stdin: &SP1Stdin) {
    // Set VM client.
    let client = ProverClient::new();
    let (_, report) = client.execute(_ELF, stdin.clone()).run().unwrap();

    // Render report.
    println!(
        "EXECUTION: # vm cycles   : {}",
        report.total_instruction_count()
    );
    println!(
        "EXECUTION: # calls to sys: {}",
        report.total_syscall_count()
    );
}

fn do_pgm_prove(_: &Args, stdin: &SP1Stdin) {
    // Set VM client.
    let client = ProverClient::new();
    let (pk, vk) = client.setup(_ELF);

    // Set proof.
    let proof = client
        .prove(&pk, stdin.clone())
        .run()
        .expect("failed to generate proof");
    println!("PROOF: generation complete");

    // Verify proof.
    client.verify(&proof, &vk).expect("failed to verify proof");
    println!("PROOF: verification complete");
}
