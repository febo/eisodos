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

## Entrypoint

Entrypoint implementation currently included in the benchmark:

- [`pinocchio`](https://github.com/anza-xyz/pinocchio)
- [`solana-program`](https://github.com/anza-xyz/agave/tree/master/sdk/program)
- [`jiminy`](https://github.com/igneous-labs/jiminy)

> [!NOTE]
> Previous benchmark included the [`solana-nostd-entrypoint`](https://github.com/cavemanloverboy/solana-nostd-entrypoint) – the project has not been archived and therefore ommitted.

| Benchmark     | `pinocchio`     | `solana-program`  | `jiminy`     |
| ------------- | --------------- | ----------------- | ------------ |
| Ping          | 🟩 **12**       | 🟥 117 (+105)      | 🟩 15 (+3)   |
| Log           | 🟩 **117**      | 🟥 222 (+105)      | 🟩 120 (+3)  |
| Account (1)   | 🟩 **32**       | 🟥 317 (+285)      | 🟩 38 (+6)   |
| Account (3)   | 🟩 **60**       | 🟥 641 (+581)      | 🟩 68 (+8)   |
| Account (5)   | 🟩 **88**       | 🟥 965 (+877)      | 🟩 98 (+10)  |
| Account (10)  | 🟩 **158**      | 🟥 1,775 (+1,617)  | 🟨 173 (+15) |
| Account (20)  | 🟩 **298**      | 🟥 3,395 (+3,097)  | 🟨 323 (+25) |
| Account (32)  | 🟩 **466**      | 🟥 5,339 (+4,873)  | 🟨 503 (+37) |
| Account (64)  | 🟩 **914**      | 🟥 10,523 (+9,609) | 🟨 983 (+69) |

> [!IMPORTANT]
> Values correspond to compute units (CUs) consumed by the entrypoint. The delta in relation to the lowest consumption is shown in brackets.
>
> Solana CLI `v2.2.15` was used in the bench tests.

## CPI and Binary Size

There are also benchmarks for CPI and binary size produced by the different entrypoints libraries. Note that these actually measure how efficient the helpers of the library are
instead of the entrypoint efficiency, since it is generally possible to improve/re-write
the helpers.


### CPI

| Benchmark (CPI)        | `pinocchio`     | `solana-program`  | `jiminy`     |
| ---------------------- | --------------- | ----------------- | ------------ |
| system::create_account | 🟩 **1,309**    | 🟥 2,866 (+1,557) | 🟩 **1,309**  |
| system::transfer       | 🟩 1,305 (+4)   | 🟥 2,459 (+1,158) | 🟩 **1,301**  |


### Binary Size

|                     | `pinocchio`     | `solana-program`    | `jiminy` |
| ------------------- | --------------- | ------------------- | -------- |
| Binary size (bytes) | 🟨 4,040 (+520) | 🟥 68,832 (+65,312) | 🟩 3,520  |

## Benchmark

The benchmark uses a simple program with multiple instructions to measure the compute units (CUs) consumed by the entrypoint. Note that the intention is not to write the most efficient program, instead to reflect an "average" program implementation. The aim is to use the exactly same program implementation, replacing the entrypoint to determine the impact on the CUs consumed.

> [!WARNING]
> This not apply to instructions that use CPIs since these involve using library specific helpers.

The program used has the following instructions:

```rust
pub enum Instruction {
    Ping,
    Log,
    Account {
        expected: u64,
    },
    CreateAccount,
    Transfer
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
