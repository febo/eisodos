use crate::{Accounts, Cpi, ProgramResult};
use jiminy_entrypoint::program_error::{BuiltInProgramError, ProgramError};
use jiminy_log::sol_log;
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
    sol_log(MSG);
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
    let [funding, new, sys_prog] = accounts.as_slice() else {
        return Err(ProgramError::from_builtin(
            BuiltInProgramError::NotEnoughAccountKeys,
        ));
    };
    let [funding, new, sys_prog] = [funding, new, sys_prog].map(|h| *h);
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
    let [from, to, sys_prog] = accounts.as_slice() else {
        return Err(ProgramError::from_builtin(
            BuiltInProgramError::NotEnoughAccountKeys,
        ));
    };
    let [from, to, sys_prog] = [from, to, sys_prog].map(|h| *h);
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
