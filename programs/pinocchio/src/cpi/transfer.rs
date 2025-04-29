use super::SYSTEM_PROGRAM_ID;
use pinocchio::{
    account_info::AccountInfo,
    cpi::invoke_signed_unchecked,
    instruction::{AccountMeta, Instruction},
    ProgramResult,
};

/// Transfer lamports between accounts.
///
/// This function is a wrapper around the system program's `transfer`
/// instruction.
///
/// # Safety
///
/// This function assumes that accounts are not mutably borrowed.
pub unsafe fn transfer_unchecked(
    from: &AccountInfo,
    to: &AccountInfo,
    lamports: u64,
) -> ProgramResult {
    // instruction accounts
    let account_metas = [
        AccountMeta::writable_signer(from.key()),
        AccountMeta::writable(to.key()),
    ];

    // instruction data
    // - [0..4 ]: instruction discriminator
    // - [4..12]: lamports amount
    let mut instruction_data = [0; 12];
    instruction_data[0] = 2;
    instruction_data[4..12].copy_from_slice(&lamports.to_le_bytes());

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
