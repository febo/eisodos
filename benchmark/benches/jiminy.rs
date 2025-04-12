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
mod jiminy {

    use super::*;
    use test::Bencher;

    #[bench]
    fn run(_bencher: &mut Bencher) {
        runner::run(&eisodos_jiminy::ID.into(), "eisodos_jiminy");
    }
}
