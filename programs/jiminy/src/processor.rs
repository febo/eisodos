use crate::{Accounts, Cpi, ProgramResult};
use jiminy_program_error::{BuiltInProgramError, ProgramError};
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
        Err(ProgramError::from_builtin(
            BuiltInProgramError::InvalidArgument,
        ))
    }
}

#[inline(always)]
pub fn process_create_account(accounts: &mut Accounts) -> ProgramResult {
    let mut accounts_itr = accounts.iter();
    let [Some(funding), Some(new), Some(sys_prog)] = core::array::from_fn(|_| accounts_itr.next())
    else {
        return Err(ProgramError::from_builtin(
            BuiltInProgramError::NotEnoughAccountKeys,
        ));
    };
    Cpi::new().invoke_signed(
        accounts,
        create_account_ix(
            sys_prog,
            CreateAccountAccounts { funding, new },
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
pub fn process_transfer(accounts: &mut Accounts) -> ProgramResult {
    let mut accounts_itr = accounts.iter();
    let [Some(from), Some(to), Some(sys_prog)] = core::array::from_fn(|_| accounts_itr.next())
    else {
        return Err(ProgramError::from_builtin(
            BuiltInProgramError::NotEnoughAccountKeys,
        ));
    };
    Cpi::new().invoke_signed(
        accounts,
        transfer_ix(sys_prog, TransferAccounts { from, to }, 1_000_000_000).as_instr(),
        &[],
    )
}
