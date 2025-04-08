pub mod runner;

use mollusk_svm::{program::keyed_account_for_system_program, Mollusk};
use solana_account::Account;
use solana_instruction::{AccountMeta, Instruction};
use solana_pubkey::Pubkey;
use std::vec;

pub const BASE_LAMPORTS: u64 = 2_000_000_000u64;

/// Create a new Mollusk instance for the given program ID and name.
pub fn setup(program_id: &Pubkey, name: &'static str) -> Mollusk {
    std::env::set_var("SBF_OUT_DIR", "../target/deploy");
    solana_logger::setup_with("");

    Mollusk::new(program_id, name)
}

/// Instructions on the program to be executed.
pub enum ProgramInstruction {
    Ping,
    Log,
    Account { expected: u64 },
    CreateAccount,
    Transfer,
}

/// Returns the instruction data for the given instruction.
pub fn instruction_data(instruction: ProgramInstruction) -> Vec<u8> {
    match instruction {
        ProgramInstruction::Ping => vec![0],
        ProgramInstruction::Log => vec![1],
        ProgramInstruction::Account { expected } => {
            let mut data = Vec::with_capacity(9);
            data.push(2);
            data.extend_from_slice(&expected.to_le_bytes());
            data
        }
        ProgramInstruction::CreateAccount => vec![3],
        ProgramInstruction::Transfer => vec![4],
    }
}

/// Generate a set of unique public keys.
pub fn generate_pubkeys(count: usize) -> Vec<Pubkey> {
    let mut keys = Vec::with_capacity(count);
    for _ in 0..count {
        keys.push(Pubkey::new_unique());
    }
    keys
}

/// Generates the instruction data and accounts for the
/// `ProgramInstruction::Account` instruction.
fn generate_account(program_id: Pubkey, expected: u64) -> (Instruction, Vec<(Pubkey, Account)>) {
    let mut keys = generate_pubkeys(expected as usize);

    let mut accounts = Vec::with_capacity(keys.len());
    let mut account_metas = Vec::with_capacity(keys.len());

    for _ in 0..keys.len() {
        let key = keys.pop().unwrap();
        accounts.push((
            key,
            Account::new(BASE_LAMPORTS, 0, &solana_system_interface::program::ID),
        ));
        account_metas.push(AccountMeta::new_readonly(key, false));
    }

    (
        Instruction {
            program_id,
            accounts: account_metas,
            data: instruction_data(crate::ProgramInstruction::Account { expected }),
        },
        accounts,
    )
}

/// Generates the instruction data and accounts for the
/// `ProgramInstruction::CreateAccount` instruction.
fn generate_create_account(program_id: Pubkey) -> (Instruction, Vec<(Pubkey, Account)>) {
    let keys = generate_pubkeys(2);
    let [key1, key2] = keys.as_slice() else {
        panic!()
    };

    let (system_program_id, system_program_account) = keyed_account_for_system_program();

    let accounts = vec![
        (
            *key1,
            Account::new(BASE_LAMPORTS, 0, &solana_system_interface::program::ID),
        ),
        // account being created, starts with 0 lamports and no data
        (
            *key2,
            Account::new(0, 0, &solana_system_interface::program::ID),
        ),
        (system_program_id, system_program_account),
    ];

    let account_metas = vec![
        AccountMeta::new(*key1, true),
        AccountMeta::new(*key2, true),
        AccountMeta::new_readonly(system_program_id, false),
    ];

    (
        Instruction {
            program_id,
            accounts: account_metas,
            data: instruction_data(crate::ProgramInstruction::CreateAccount),
        },
        accounts,
    )
}

/// Generates the instruction data and accounts for the
/// `ProgramInstruction::Transfer` instruction.
fn generate_transfer(program_id: Pubkey) -> (Instruction, Vec<(Pubkey, Account)>) {
    let keys = generate_pubkeys(2);
    let [key1, key2] = keys.as_slice() else {
        panic!()
    };

    let (system_program_id, system_program_account) = keyed_account_for_system_program();

    let accounts = vec![
        (
            *key1,
            Account::new(BASE_LAMPORTS, 0, &solana_system_interface::program::ID),
        ),
        // account receiving the transfer, so it starts with 0 lamports
        (
            *key2,
            Account::new(0, 0, &solana_system_interface::program::ID),
        ),
        (system_program_id, system_program_account),
    ];

    let account_metas = vec![
        AccountMeta::new(*key1, true),
        AccountMeta::new(*key2, true),
        AccountMeta::new_readonly(system_program_id, false),
    ];

    (
        Instruction {
            program_id,
            accounts: account_metas,
            data: instruction_data(crate::ProgramInstruction::Transfer),
        },
        accounts,
    )
}
