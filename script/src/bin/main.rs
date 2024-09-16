use clap::Parser;
use hex;
use sp1_sdk::{ProverClient, SP1Stdin};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const _ELF: &[u8] = include_bytes!("../../../elf/riscv32im-succinct-zkvm-elf");

/// Test fixtures.
/// TODO: read from fsys.
const BLAKE2B_DIGEST: &str = "44682ea86b704fb3c65cd16f84a76b621e04bbdb3746280f25cf062220e471b4";
const BLAKE2B_MSG: &str = "أبو يوسف يعقوب بن إسحاق الصبّاح الكندي‎";
const ED25519_MSG: &str = "44682ea86b704fb3c65cd16f84a76b621e04bbdb3746280f25cf062220e471b4";
const ED25519_PBK: &str = "764f83295812c03354e0cd64718a7e50b452696799dc9d6e446338d668f3b2d9";
const _ED25519_PVK: &str = "e541eed0cc0fa324fa8f34ab6faa186d98826777344c5518819fed5e20448282";
const ED25519_SIG: &str = "2fa8e929a7514496545d098e86841463ef66358ff0930073fde3b138f66a2cef5304d884baa693a971d002d7e071f658fb16de8c1e5c80ba5ecea8b3866f8106";
const SECP256K1_MSG: &str = "44682ea86b704fb3c65cd16f84a76b621e04bbdb3746280f25cf062220e471b4";
const SECP256K1_PBK: &str = "03eed4eb0b40b3131679c365e3a23780eabfeaeb01776b0f908223ad1d4bd06f0d";
const _SECP256K1_PVK: &str = "0b0e6094077ebae48bc119b35a134890424e0e5dbb71e667edae1978c49e7089";
const SECP256K1_SIG: &str = "ba07224f974d973682d93dca27517b2bbbacca42df551ef22be3f80c65bc4923bad512c31a85f996345be08a8a2557f814cefc04c3e8a8ddb58c3749a65d4ce4";

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
    let mut stdin = SP1Stdin::new();
    stdin.write_vec(hex::decode(ED25519_MSG).unwrap());
    stdin.write_vec(hex::decode(ED25519_SIG).unwrap());
    stdin.write_vec(hex::decode(ED25519_PBK).unwrap());
    stdin.write_vec(hex::decode(SECP256K1_MSG).unwrap());
    stdin.write_vec(hex::decode(SECP256K1_SIG).unwrap());
    stdin.write_vec(hex::decode(SECP256K1_PBK).unwrap());
    stdin.write_vec(BLAKE2B_MSG.as_bytes().to_vec());
    stdin.write_vec(hex::decode(BLAKE2B_DIGEST).unwrap());

    // Execute | Prove.
    if args.execute {
        do_execute(stdin);
    } else {
        do_prove(stdin);
    }
}

fn do_execute(stdin: SP1Stdin) {
    // Set client & execute program.
    let client = ProverClient::new();
    let (output, report) = client.execute(_ELF, stdin).run().unwrap();

    // Process output.
    // TODO

    // Process report.
    println!("Number of cycles: {}", report.total_instruction_count());
    println!("Number of sys calls: {}", report.total_syscall_count());
}

fn do_prove(stdin: SP1Stdin) {
    // Set client & prove program execution.
    let client = ProverClient::new();
    let (pk, vk) = client.setup(_ELF);

    // Set proof.
    let proof = client
        .prove(&pk, stdin)
        .run()
        .expect("failed to generate proof");
    println!("Successfully generated proof!");

    // Verify proof.
    client.verify(&proof, &vk).expect("failed to verify proof");
    println!("Successfully verified proof!");
}
