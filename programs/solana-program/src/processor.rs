use solana_account_info::AccountInfo;
use solana_cpi::invoke;
use solana_msg::msg;
use solana_program_error::{ProgramError, ProgramResult};

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
pub fn process_account(accounts: &[AccountInfo], expected: u64) -> ProgramResult {
    if accounts.len() == expected as usize {
        Ok(())
    } else {
        Err(ProgramError::InvalidArgument)
    }
}

#[inline(always)]
pub fn process_create_account(accounts: &[AccountInfo]) -> ProgramResult {
    invoke(
        &solana_system_interface::instruction::create_account(
            accounts[0].key,
            accounts[1].key,
            500_000_000,
            10,
            &crate::ID,
        ),
        &[accounts[0].clone(), accounts[1].clone()],
    )
}

#[inline(always)]
pub fn process_transfer(accounts: &[AccountInfo]) -> ProgramResult {
    invoke(
        &solana_system_interface::instruction::transfer(
            accounts[0].key,
            accounts[1].key,
            1_000_000_000,
        ),
        &[accounts[0].clone(), accounts[1].clone()],
    )
}
