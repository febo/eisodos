<h1 align="center">
  <code>eisodos</code>
</h1>
<p align="center">
  <img width="400" alt="eisodos" src="https://github.com/user-attachments/assets/c3799ce0-a432-4898-b98c-869458a06439" />
</p>
<p align="center">
  A simple benchmark of SVM entrypoints.
</p>

## Overview

The purpose of `eisodos` is to offer a simple benchmark of different program entrypoint implementations. An entrypoint is used to parse the [SBF input](https://solana.com/docs/programs/faq#input-parameter-serialization) for a program, providing the information of an instruction input in a "friendly" way. The SBF loader passes the input parameters as a byte array and the entrypoint then transforms the input into separate typed entities &mdash; `program id`, `accounts` array and `instruction data`.

## Entrypoints

Entrypoint implementation currently included in the benchmark:

- [`pinocchio`](https://github.com/anza-xyz/pinocchio)
- [`solana-nostd-entrypoint`](https://github.com/cavemanloverboy/solana-nostd-entrypoint)
- [`solana-program`](https://github.com/anza-xyz/agave/tree/master/sdk/program)
- [`jiminy`](https://github.com/igneous-labs/jiminy)

| Benchmark     | `pinocchio`     | `solana-nostd-entrypoint` | `solana-program`  | `jiminy`     |
| ------------- | --------------- | ------------------------- | ----------------- | ------------ |
| _Entrypoint_  |
| Ping          | 🟩 **14**       | 🟩 **14**                 | 🟧 41 (+27)       | 🟩 **14**    |
| Log           | 🟩 **119**      | 🟩 **119**                | 🟧 146 (+27)      | 🟩 **119**   |
| Account (1)   | 🟩 38 (+2)      | 🟩 39 (+3)                | 🟥 235 (+199)     | 🟩 **36**    |
| Account (3)   | 🟩 **66**       | 🟩 69 (+3)                | 🟥 541 (+475)     | 🟩 **66**    |
| Account (5)   | 🟩 **94**       | 🟩 99 (+5)                | 🟥 847 (+751)     | 🟩 96 (+2)   |
| Account (10)  | 🟩 **164**      | 🟩 174 (+10)              | 🟥 1,612 (+1,441) | 🟩 171 (+7)  |
| Account (20)  | 🟩 **304**      | 🟨 324 (+20)              | 🟥 3,142 (+2,821) | 🟨 321 (+17) |
| Account (32)  | 🟩 **472**      | 🟨 504 (+32)              | 🟥 4,978 (+4,477) | 🟨 501 (+29) |
| Account (64)  | 🟩 **920**      | 🟨 985 (+65)              | 🟥 9,874 (+8,893) | 🟨 981 (+61) |
| _CPI_         |
| CreateAccount | 🟨 1,449 (+142) | 🟨 1,494 (+187)           | 🟥 2,786 (+1,479) | 🟩 **1,307** |
| Transfer      | 🟨 1,439 (+140) | 🟨 1,487 (+180)           | 🟥 2,379 (+1,080) | 🟩 **1,299** |

> [!IMPORTANT]
> Values correspond to compute units (CUs) consumed by the entrypoint. The delta in relation to the lowest consumption is shown in brackets.
>
> Solana CLI `v2.2.6` was used in the bench tests.

## Binary Sizes

The size of the compiled binary for the benchmark program is shown below. The delta in relation to the smalles binary size is shown in brackets.

| Binary size (bytes) | `pinocchio`        | `solana-nostd-entrypoint` | `solana-program`    | `jiminy` |
| ------------------- | ------------------ | ------------------------- | ------------------- | -------- |
|                     | 🟥 10,736 (+7,240) | 🟥 17,720 (+14,224)       | 🟥 64,688 (+61,192) | 🟩 3,496 |

## Benchmark

The benchmark uses a simple program with multiple instructions to measure the compute units (CUs) consumed by the entrypoint. Note that the intention is not to write the most efficient program, instead to reflect an "average" program implementation. The aim is to use the exactly same program implementation, replacing the entrypoint to determine the impact on the CUs consumed.

The program used has the following instructions:

```rust
pub enum Instruction {
    Ping,
    Log,
    Account {
        expected: u64,
    }
}
```

### Instructions

#### `Ping`

This instruction has an empty processor and does not expect any account. The only data passed to the program is the instruction discriminator (`0` in this case).

#### `Log`

Similar to the `Ping` instruction, this instruction does not expect any account and only logs a static message.

#### `Account`

This instruction receives an `u64` value as part of the instruction data, which specifies the number of accounts expected by the processor. The processor only asserts that the number of accounts received is the same as the `expected` value. This in essence measures how much CUs the entrypoint comsumes to parse the input accounts.

#### `CreateAccount`

This instruction receives 3 accounts (`from`, `account` and `system_program`) and performs a CPI to the System program to create the `account` with `500_000_000` lamports and `10` bytes of account data. These values are fixed on the processor.

#### `Transfer`

This instruction receives 3 accounts (`from`, `to` and `system_program`) and performs a CPI to the System program to transfer `1_000_000_000` lamports. The lamports amount is fixed.

### Program

The program is structure in 4 different source files:

- `entrypoint.rs`: includes the entrypoint definition and "dispatches" the instruction to the corresponding processor.

- `instruction.rs`: defines the instructions available on the program and the parsing logic for the input instruction data.

- `lib.rs`: defines the modules of the program and the program ID.

- `processor.rs`: includes the processor for each instruction.

The implementation across all different entrypoint programs is as similar as possible. In most cases, the only differences are on the types import, since each entrypoint defines their own `AccountInfo` and/or `Pubkey` types.

## Evaluation

The evaluation is performed using [`mollusk`](https://github.com/buffalojoec/mollusk).

To run the benchmarks, you will need to build the programs. After cloning the repository, run:

```bash
pnpm install
```

This will install the required packages. Then all programs can be buiit using:

```bash
pnpm programs:build
```

After this, you are ready to run individual benchmarks by using:

```bash
cargo bench --bench <ENTRYPOINT_NAME>
```

The `ENTRYPOINT_NAME` will be one of `pinocchio`, `solana_nostd_entrypoint` or `solana_program`.

The results are written to `./target/benches/compute_units.md`. Each execution is described by 3 columns:

- `Name`: name of the benchmark; this will specify the name of the instruction and the parameters used.

- `CUs`: number of compute units consumed by the execution.

- `Delta`: the difference in compute units between latest benchmark and the previous; this will provide a quick way to assess the differences between entrypoints.

The results of an execution are compared to the previous one (if there is one), with delta differences shown after a `+` and `-` symbol.
