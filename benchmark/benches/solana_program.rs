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
mod solana_program {

    use super::*;
    use test::Bencher;

    #[bench]
    fn run(_bencher: &mut Bencher) {
        runner::run(&eisodos_solana_program::ID, "eisodos_solana_program");
    }
}
