use crate::{Accounts, ProgramResult, MAX_ACCOUNTS};
use jiminy_entrypoint::entrypoint;

entrypoint!(process_instruction, MAX_ACCOUNTS);

#[inline(always)]
pub fn process_instruction(
    _accounts: &mut Accounts,
    _instruction_data: &[u8],
    _program_id: &[u8; 32],
) -> ProgramResult {
    core::hint::black_box(Ok(()))
}
