use crate::{
    instruction::Instruction,
    processor::{
        process_account, process_create_account, process_log, process_ping, process_transfer,
    },
};
use solana_account_info::AccountInfo;
use solana_program_error::ProgramResult;
use solana_pubkey::Pubkey;

solana_program_entrypoint::entrypoint!(process_instruction);

#[inline(always)]
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
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
