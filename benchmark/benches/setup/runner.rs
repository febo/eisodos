use super::{generate_account, setup};
use mollusk_svm_bencher::MolluskComputeUnitBencher;
use solana_pubkey::Pubkey;

pub fn run(program_id: &Pubkey, name: &'static str) {
    let mollusk = setup(program_id, name);
    let mut bencher = MolluskComputeUnitBencher::new(mollusk)
        .must_pass(true)
        .out_dir("../target/benches");

    // Account 0

    let (instruction, accounts) = generate_account(*program_id, 0);
    bencher = bencher.bench(("Account (0)", &instruction, &accounts));

    // Account 1

    let (instruction, accounts) = generate_account(*program_id, 1);
    bencher = bencher.bench(("Account (1)", &instruction, &accounts));

    // Account 2

    let (instruction, accounts) = generate_account(*program_id, 2);
    bencher = bencher.bench(("Account (2)", &instruction, &accounts));

    // Account 3

    let (instruction, accounts) = generate_account(*program_id, 3);
    bencher = bencher.bench(("Account (3)", &instruction, &accounts));

    // Account 4

    let (instruction, accounts) = generate_account(*program_id, 4);
    bencher = bencher.bench(("Account (4)", &instruction, &accounts));

    // Account 5

    let (instruction, accounts) = generate_account(*program_id, 5);
    bencher = bencher.bench(("Account (5)", &instruction, &accounts));

    // Account 6

    let (instruction, accounts) = generate_account(*program_id, 6);
    bencher = bencher.bench(("Account (6)", &instruction, &accounts));

    // Account 7

    let (instruction, accounts) = generate_account(*program_id, 7);
    bencher = bencher.bench(("Account (7)", &instruction, &accounts));

    // Account 8

    let (instruction, accounts) = generate_account(*program_id, 8);
    bencher = bencher.bench(("Account (8)", &instruction, &accounts));

    // Account 16

    let (instruction, accounts) = generate_account(*program_id, 16);
    bencher = bencher.bench(("Account (16)", &instruction, &accounts));

    // Account 32

    let (instruction, accounts) = generate_account(*program_id, 32);
    bencher = bencher.bench(("Account (32)", &instruction, &accounts));

    // Account 64

    let (instruction, accounts) = generate_account(*program_id, 64);
    bencher = bencher.bench(("Account (64)", &instruction, &accounts));

    // Run the benchmarks.

    bencher.execute();
}
