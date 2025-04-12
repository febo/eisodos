use crate::{AccountHandles, Accounts, ProgramResult, MAX_ACCOUNTS, MAX_CPI_ACCOUNTS};
use jiminy_cpi::invoke_signed;
use jiminy_program_error::ProgramError;
use jiminy_system_prog_interface::{
    create_account_ix, transfer_ix, CreateAccountAccounts, CreateAccountIxArgs, TransferAccounts,
};

#[inline(always)]
pub fn process_ping() -> ProgramResult {
    Ok(())
}

#[inline(always)]
pub fn process_log() -> ProgramResult {
    const MSG: &str = "Instruction: Log";

    unsafe {
        jiminy_syscall::sol_log_(MSG.as_ptr(), MSG.len() as u64);
    }
    Ok(())
}

#[inline(always)]
pub fn process_account(accounts: &Accounts, expected: u64) -> ProgramResult {
    if accounts.len() == expected as usize {
        Ok(())
    } else {
        Err(ProgramError::InvalidArgument)
    }
}

#[inline(always)]
pub fn process_create_account(accounts: &mut Accounts, handles: &AccountHandles) -> ProgramResult {
    // TODO: need to export const generic from jiminy_system_prog_interface
    invoke_signed::<MAX_ACCOUNTS, MAX_CPI_ACCOUNTS>(
        accounts,
        create_account_ix(
            handles[2],
            CreateAccountAccounts {
                funding: handles[0],
                new: handles[1],
            },
            CreateAccountIxArgs {
                lamports: 500_000_000,
                space: 10,
                owner: crate::ID,
            },
        )
        .as_instr(),
        &[],
    )
}

#[inline(always)]
pub fn process_transfer(accounts: &mut Accounts, handles: &AccountHandles) -> ProgramResult {
    // TODO: need to export const generic from jiminy_system_prog_interface
    invoke_signed::<MAX_ACCOUNTS, MAX_CPI_ACCOUNTS>(
        accounts,
        transfer_ix(
            handles[2],
            TransferAccounts {
                from: handles[0],
                to: handles[1],
            },
            1_000_000_000,
        )
        .as_instr(),
        &[],
    )
}
