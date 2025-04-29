use super::SYSTEM_PROGRAM_ID;
use pinocchio::{
    account_info::AccountInfo,
    cpi::invoke_signed_unchecked,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    ProgramResult,
};

/// Create a new account.
///
/// This function is a wrapper around the system program's `create_account`
/// instruction.
///
/// # Safety
///
/// This function assumes that accounts are not mutably borrowed.
pub unsafe fn create_account_unchecked(
    from: &AccountInfo,
    to: &AccountInfo,
    lamports: u64,
    space: u64,
    owner: &Pubkey,
) -> ProgramResult {
    // instruction accounts
    let account_metas = [
        AccountMeta::writable_signer(from.key()),
        AccountMeta::writable_signer(to.key()),
    ];

    // instruction data
    // - [0..4  ]: instruction discriminator
    // - [4..12 ]: lamports
    // - [12..20]: account space
    // - [20..52]: owner pubkey
    let mut instruction_data = [0; 52];
    // create account instruction has a '0' discriminator
    instruction_data[4..12].copy_from_slice(&lamports.to_le_bytes());
    instruction_data[12..20].copy_from_slice(&space.to_le_bytes());
    instruction_data[20..52].copy_from_slice(owner);

    // SAFETY: Accounts are in the correct order since the helper created
    // the instruction accounts array. The caller must guarantee that accounts
    // are not mutably borrowed.
    unsafe {
        invoke_signed_unchecked(
            &Instruction {
                program_id: &SYSTEM_PROGRAM_ID,
                accounts: &account_metas,
                data: &instruction_data,
            },
            &[from.into(), to.into()],
            &[],
        );
    }

    Ok(())
}
