#![feature(test)]

extern crate mollusk_svm;
extern crate mollusk_svm_bencher;
extern crate solana_account;
extern crate solana_instruction;
extern crate solana_pubkey;
extern crate test;

mod setup;
use setup::*;

#[cfg(test)]
mod pinocchio {

    use super::*;
    use solana_pubkey::Pubkey;
    use test::Bencher;

    pub const ID: Pubkey = Pubkey::from_str_const("Pinocchio1111111111111111111111111111111111");

    #[bench]
    fn run(_bencher: &mut Bencher) {
        runner::run(&ID, "eisodos_pinocchio");
    }
}
