use crate::{
    instruction::Instruction,
    processor::{
        process_account, process_create_account, process_log, process_ping, process_transfer,
    },
    Accounts, ProgramResult, MAX_ACCOUNTS,
};
use jiminy_entrypoint::entrypoint;

entrypoint!(process_instruction, MAX_ACCOUNTS);

#[inline(always)]
pub fn process_instruction(
    accounts: &mut Accounts,
    instruction_data: &[u8],
    _program_id: &[u8; 32],
) -> ProgramResult {
    let instruction = Instruction::unpack(instruction_data)?;

    match instruction {
        Instruction::Ping => process_ping(),
        Instruction::Log => process_log(),
        Instruction::Account { expected } => process_account(accounts, expected),
        Instruction::CreateAccount => process_create_account(accounts),
        Instruction::Transfer => process_transfer(accounts),
    }
}
