#![feature(test)]

extern crate mollusk_svm;
extern crate mollusk_svm_bencher;
extern crate solana_sdk;
extern crate test;

mod setup;
use setup::*;

#[cfg(test)]
mod solana_nostd_entrypoint {

    use super::*;
    use test::Bencher;

    #[bench]
    fn run(_bencher: &mut Bencher) {
        runner::run(
            &eisodos_solana_nostd_entrypoint::ID,
            "eisodos_solana_nostd_entrypoint",
        );
    }
}
