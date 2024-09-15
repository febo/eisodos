use {
    crate::{
        instruction::Instruction,
        processor::{process_account, process_log, process_ping},
    },
    solana_nostd_entrypoint::{entrypoint_nostd, NoStdAccountInfo},
    solana_program::{entrypoint::ProgramResult, pubkey::Pubkey},
};

entrypoint_nostd!(process_instruction, 64);

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
    }
}
