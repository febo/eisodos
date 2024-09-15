pub mod runner;

use mollusk_svm::Mollusk;
use solana_sdk::{
    account::AccountSharedData,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};

pub const BASE_LAMPORTS: u64 = 1_000_000_000u64;

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

/// Generates the instructions and accounts for the
/// `ProgramInstruction::Account` instruction.
fn generate_account(
    program_id: Pubkey,
    expected: u64,
) -> (Instruction, Vec<(Pubkey, AccountSharedData)>) {
    let mut keys = generate_pubkeys(expected as usize);

    let mut accounts = Vec::with_capacity(keys.len());
    let mut account_metas = Vec::with_capacity(keys.len());

    for _ in 0..keys.len() {
        let key = keys.pop().unwrap();
        accounts.push((
            key,
            AccountSharedData::new(BASE_LAMPORTS, 0, &system_program::ID),
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
