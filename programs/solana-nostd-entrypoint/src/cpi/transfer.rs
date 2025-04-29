use super::invoke_unchecked;
use solana_nostd_entrypoint::{InstructionC, NoStdAccountInfo};
use solana_program::{entrypoint::ProgramResult, system_program};

/// Transfer lamports between accounts.
///
/// This function is a wrapper around the system program's `transfer`
/// instruction.
///
/// # Safety
///
/// This function assumes that accounts are not mutably borrowed.
pub unsafe fn transfer_unchecked(
    from: &NoStdAccountInfo,
    to: &NoStdAccountInfo,
    lamports: u64,
) -> ProgramResult {
    // instruction data
    // - [0..4  ]: instruction discriminator
    // - [4..12 ]: lamports
    let mut instruction_data = [0; 12];
    instruction_data[0] = 2;
    instruction_data[4..12].copy_from_slice(&lamports.to_le_bytes());

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
