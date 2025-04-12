use jiminy_program_error::ProgramError;

pub mod entrypoint;
pub mod instruction;
pub mod processor;

type ProgramResult = Result<(), ProgramError>;

const MAX_ACCOUNTS: usize = 64;
// TODO: this sucks
const MAX_CPI_ACCOUNTS: usize = 55;

type Accounts<'a> = jiminy_account::Accounts<'a, MAX_ACCOUNTS>;

type AccountHandles<'a> = jiminy_account::AccountHandles<'a, MAX_ACCOUNTS>;

const PROG_ID_STR: &str = "Jim1ny1111111111111111111111111111111111111";

pub const ID: [u8; 32] = const_crypto::bs58::decode_pubkey(PROG_ID_STR);
