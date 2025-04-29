use super::invoke_unchecked;
use solana_nostd_entrypoint::{InstructionC, NoStdAccountInfo};
use solana_program::{entrypoint::ProgramResult, pubkey::Pubkey, system_program};

/// Create a new account.
///
/// This function is a wrapper around the system program's `create_account`
/// instruction.
///
/// # Safety
///
/// This function assumes that accounts are not mutably borrowed.
pub unsafe fn create_account_unchecked(
    from: &NoStdAccountInfo,
    to: &NoStdAccountInfo,
    lamports: u64,
    space: u64,
    owner: &Pubkey,
) -> ProgramResult {
    // instruction data
    // - [0..4  ]: instruction discriminator
    // - [4..12 ]: lamports
    // - [12..20]: account space
    // - [20..52]: owner pubkey
    let mut instruction_data = [0; 52];
    // create account instruction has a '0' discriminator
    instruction_data[4..12].copy_from_slice(&lamports.to_le_bytes());
    instruction_data[12..20].copy_from_slice(&space.to_le_bytes());
    instruction_data[20..52].copy_from_slice(owner.as_ref());

    let instruction_accounts = [from.to_meta_c_signer(), to.to_meta_c_signer()];

    invoke_unchecked(
        &InstructionC {
            program_id: &system_program::ID,
            accounts: instruction_accounts.as_ptr(),
            accounts_len: instruction_accounts.len() as u64,
            data: instruction_data.as_ptr(),
            data_len: instruction_data.len() as u64,
        },
        &[from, to],
    )
}
