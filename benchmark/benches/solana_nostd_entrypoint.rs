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
mod solana_nostd_entrypoint {

    use super::*;
    use solana_pubkey::Pubkey;
    use test::Bencher;

    #[bench]
    fn run(_bencher: &mut Bencher) {
        let id = eisodos_solana_nostd_entrypoint::ID;
        runner::run(
            &Pubkey::new_from_array(id.to_bytes()),
            "eisodos_solana_nostd_entrypoint",
        );
    }
}
