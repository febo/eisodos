use crate::processor::{process_create_account, process_transfer};
#[allow(unused_imports)]
use {
    crate::{
        instruction::Instruction,
        processor::{process_account, process_log, process_ping},
    },
    solana_nostd_entrypoint::{
        basic_panic_impl, entrypoint_nostd, noalloc_allocator, NoStdAccountInfo,
    },
    solana_program::{entrypoint::ProgramResult, log, pubkey::Pubkey},
};

entrypoint_nostd!(process_instruction, 64);
noalloc_allocator!();
basic_panic_impl!();

#[inline(always)]
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[NoStdAccountInfo],
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
