use crate::{Accounts, Cpi, ProgramResult};
use jiminy_program_error::{BuiltInProgramError, ProgramError};
use jiminy_system_prog_interface::{
    create_account_ix, transfer_ix, CreateAccountIxAccounts, CreateAccountIxData,
    TransferIxAccounts, TransferIxData,
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
            CreateAccountIxAccounts::memset(sys_prog)
                .with_funding(funding)
                .with_new(new),
            &CreateAccountIxData::new(500_000_000, 10, &crate::ID),
        ),
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
        transfer_ix(
            sys_prog,
            TransferIxAccounts::memset(sys_prog)
                .with_from(from)
                .with_to(to),
            &TransferIxData::new(1_000_000_000),
        ),
        &[],
    )
}
