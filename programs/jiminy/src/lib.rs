use jiminy_entrypoint::program_error::ProgramError;

pub mod entrypoint;
pub mod instruction;
pub mod processor;

type ProgramResult = Result<(), ProgramError>;

const MAX_ACCOUNTS: usize = 128;
// TODO: CPI takes up way too much stack space
const MAX_CPI_ACCOUNTS: usize = 36;

type Cpi = jiminy_cpi::Cpi<MAX_CPI_ACCOUNTS>;

type Accounts<'a> = jiminy_entrypoint::account::Accounts<'a, MAX_ACCOUNTS>;

const PROG_ID_STR: &str = "Jim1ny1111111111111111111111111111111111111";

pub const ID: [u8; 32] = const_crypto::bs58::decode_pubkey(PROG_ID_STR);
