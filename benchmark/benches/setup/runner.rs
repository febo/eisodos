use super::{generate_account, instruction_data, setup};
use mollusk_svm_bencher::MolluskComputeUnitBencher;
use solana_sdk::{instruction::Instruction, pubkey::Pubkey};

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
    bencher = bencher.bench(("ping", &instruction, &[]));

    // Log

    let instruction = Instruction {
        program_id: *program_id,
        accounts: vec![],
        data: instruction_data(crate::ProgramInstruction::Log),
    };
    bencher = bencher.bench(("log", &instruction, &[]));

    // Account 1

    let (instruction, accounts) = generate_account(*program_id, 1);
    bencher = bencher.bench(("u64 data + 1 account", &instruction, &accounts));

    // Account 5

    let (instruction, accounts) = generate_account(*program_id, 5);
    bencher = bencher.bench(("u64 data + 5 accounts", &instruction, &accounts));

    // InstructionData 10

    let (instruction, accounts) = generate_account(*program_id, 10);
    bencher = bencher.bench(("u64 data + 10 accounts", &instruction, &accounts));

    // InstructionData 20

    let (instruction, accounts) = generate_account(*program_id, 20);
    bencher = bencher.bench(("u64 data + 20 accounts", &instruction, &accounts));

    // InstructionData 32

    let (instruction, accounts) = generate_account(*program_id, 32);
    bencher = bencher.bench(("u64 data + 32 accounts", &instruction, &accounts));

    // InstructionData 64

    let (instruction, accounts) = generate_account(*program_id, 64);
    bencher = bencher.bench(("u64 data + 64 accounts", &instruction, &accounts));

    // Run the benchmarks.

    bencher.execute();
}
