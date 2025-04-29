use pinocchio::pubkey::Pubkey;

mod create_account;
mod transfer;

pub use create_account::create_account_unchecked;
pub use transfer::transfer_unchecked;

const SYSTEM_PROGRAM_ID: Pubkey = [0; 32];
