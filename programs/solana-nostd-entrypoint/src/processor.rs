use {
    crate::cpi::{create_account, transfer},
    solana_nostd_entrypoint::NoStdAccountInfo,
    solana_program::{entrypoint::ProgramResult, msg, program_error::ProgramError},
};

#[inline(always)]
pub fn process_ping() -> ProgramResult {
    Ok(())
}

#[inline(always)]
pub fn process_log() -> ProgramResult {
    msg!("Instruction: Log");
    Ok(())
}

#[inline(always)]
pub fn process_account(accounts: &[NoStdAccountInfo], expected: u64) -> ProgramResult {
    if accounts.len() == expected as usize {
        Ok(())
    } else {
        Err(ProgramError::InvalidArgument)
    }
}

#[inline(always)]
pub fn process_create_account(accounts: &[NoStdAccountInfo]) -> ProgramResult {
    create_account(&accounts[0], &accounts[1], 500_000_000, 10, &crate::ID)
}

#[inline(always)]
pub fn process_transfer(accounts: &[NoStdAccountInfo]) -> ProgramResult {
    transfer(&accounts[0], &accounts[1], 1_000_000_000)
}
