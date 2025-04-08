use super::{
    generate_account, generate_create_account, generate_transfer, instruction_data, setup,
};
use mollusk_svm_bencher::MolluskComputeUnitBencher;
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;

pub fn run(program_id: &Pubkey, name: &'static str) {
    let mollusk = setup(program_id, name);
    let mut bencher = MolluskComputeUnitBencher::new(mollusk)
        .must_pass(true)
        .out_dir("../target/benches");

    // Ping

    let instruction = Instruction {
        program_id: *program_id,
        accounts: vec![],
        data: instruction_data(crate::ProgramInstruction::Ping),
    };
    bencher = bencher.bench(("Ping", &instruction, &[]));

    // Log

    let instruction = Instruction {
        program_id: *program_id,
        accounts: vec![],
        data: instruction_data(crate::ProgramInstruction::Log),
    };
    bencher = bencher.bench(("Log", &instruction, &[]));

    // Account 1

    let (instruction, accounts) = generate_account(*program_id, 1);
    bencher = bencher.bench(("Account (1)", &instruction, &accounts));

    // Account 3

    let (instruction, accounts) = generate_account(*program_id, 3);
    bencher = bencher.bench(("Account (3)", &instruction, &accounts));

    // Account 5

    let (instruction, accounts) = generate_account(*program_id, 5);
    bencher = bencher.bench(("Account (5)", &instruction, &accounts));

    // Account 10

    let (instruction, accounts) = generate_account(*program_id, 10);
    bencher = bencher.bench(("Account (10)", &instruction, &accounts));

    // Account 20

    let (instruction, accounts) = generate_account(*program_id, 20);
    bencher = bencher.bench(("Account (20)", &instruction, &accounts));

    // Account 32

    let (instruction, accounts) = generate_account(*program_id, 32);
    bencher = bencher.bench(("Account (32)", &instruction, &accounts));

    // Account 64

    let (instruction, accounts) = generate_account(*program_id, 64);
    bencher = bencher.bench(("Account (64)", &instruction, &accounts));

    // CreateAccount

    let (instruction, accounts) = generate_create_account(*program_id);
    bencher = bencher.bench(("CreateAccount", &instruction, &accounts));

    // Transfer

    let (instruction, accounts) = generate_transfer(*program_id);
    bencher = bencher.bench(("Transfer", &instruction, &accounts));

    // Run the benchmarks.

    bencher.execute();
}
