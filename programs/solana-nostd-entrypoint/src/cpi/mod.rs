use solana_nostd_entrypoint::{AccountInfoC, InstructionC, NoStdAccountInfo};
use solana_program::{entrypoint::ProgramResult, program_error::ProgramError};
use std::mem::MaybeUninit;

pub mod create_account;
pub mod transfer;

pub use create_account::*;
pub use transfer::*;

/// Helper function to invoke a program.
///
/// The helpers performs the following checks:
/// * The number of accounts passed to the instruction is equal to the number of
///   accounts expected by the instruction.
/// * The accounts passed to the instruction are the same as the accounts info
///   (`pubkey` comparison).
/// * The accounts are borrowed as mutable if the instruction is writable.
///
/// These checks are similar to the checks performed by the default invoke in
/// `solana_program`.
///
/// # Safety
///
/// This function assumes that accounts are not mutably borrowed and passed
/// in the correct order.
unsafe fn invoke_unchecked<const ACCOUNTS: usize>(
    instruction: &InstructionC,
    accounts: &[&NoStdAccountInfo; ACCOUNTS],
) -> ProgramResult {
    if (instruction.accounts_len as usize) < ACCOUNTS {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    const UNINIT: MaybeUninit<AccountInfoC> = MaybeUninit::<AccountInfoC>::uninit();
    let mut infos = [UNINIT; ACCOUNTS];
    infos
        .iter_mut()
        .zip(accounts.iter())
        .for_each(|(info, account)| {
            info.write(account.to_info_c());
        });

    let seeds: &[&[&[u8]]] = &[];

    #[cfg(target_os = "solana")]
    unsafe {
        solana_program::syscalls::sol_invoke_signed_c(
            instruction as *const InstructionC as *const u8,
            infos.as_ptr() as *const u8,
            infos.len() as u64,
            seeds.as_ptr() as *const u8,
            seeds.len() as u64,
        );
    }

    // For clippy
    #[cfg(not(target_os = "solana"))]
    core::hint::black_box(&(&instruction, &accounts, &seeds));

    Ok(())
}
