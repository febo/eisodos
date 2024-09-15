use solana_program::program_error::ProgramError;

#[derive(Clone, Debug)]
#[rustfmt::skip]
pub enum Instruction {
    Ping,
    Log,
    Account {
        expected: u64,
    }
}

impl Instruction {
    /// Unpacks a byte buffer into a [Instruction](enum.Instruction.html).
    #[inline(always)]
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        match input.split_first() {
            // 0 - Ping
            Some((&0, [])) => Ok(Instruction::Ping),
            // 1 - Log
            Some((&1, [])) => Ok(Instruction::Log),
            // 2 - Account
            Some((&2, remaining)) if remaining.len() == 8 => Ok(Instruction::Account {
                expected: u64::from_le_bytes(remaining[0..8].try_into().unwrap()),
            }),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
