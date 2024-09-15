<h1 align="center">
  <code>eisodos</code>
</h1>
<p align="center">
  <img width="400" alt="eisodos" src="https://github.com/user-attachments/assets/todo" />
</p>
<p align="center">
  A simple benchmark of SVM entrypoints.
</p>

## Overview

The purpose of `eisodos` is to offer a simple benchmark of different program entrypoint implementations. An entrypoint is used to parse the [SBF input](https://solana.com/docs/programs/faq#input-parameter-serialization) for a program, providing the information regarding accounts and instruction data in a "friendly" way. The SBF loader passes the input parameters as a byte array and the entrypoint then transforms the input into separate typed entities &mdash; `program id`, `accounts` array and `instruction data`.

## Entrypoints

Entrypoint implementation included in the benchmark:

* [`pinocchio`](https://github.com/febo/pinocchio)
* [`solana-nostd-entrypoint`](https://github.com/cavemanloverboy/solana-nostd-entrypoint)
* [`solana-program`](https://github.com/anza-xyz/agave/tree/master/sdk/program)

| Benchmark              | `pinocchio` | `solana-nostd-entrypoint` | `solana-program` |
|------------------------|-------------|---------------------------|------------------|
| ping                   | 18          | 18                        | 47 (+29)         |
| log                    | 121         | 121                       | 150 (+29)        |
| u64 data + 1 account   | 44          | 50 (+6)                   | 250 (+206)       |
| u64 data + 5 accounts  | 100         | 142 (+42)                 | 898 (+798)       |
| u64 data + 10 accounts | 170         | 257 (+87)                 | 1,708 (+1,538)   | 
| u64 data + 20 accounts | 310         | 487 (+177)                | 3,328 (+3,018)   |
| u64 data + 32 accounts | 478         | 763 (+285)                | 5,272 (+4,794)   |
| u64 data + 64 accounts | 926         | 1,499 (+573)              | 10,456 (+9,530)  |

> [!NOTE]
> Values correspond to CUs consumed. The delta in relation to the lowest consumption is shown in brackets.

## Benchmark

The benchmark uses a simple program with multiple instructions to measure the compute units (CU) consumed by the entrypoint. Note that the intention is not to write the most efficient program. The aim instead is to use the exactly same program implementation, replacing the entrypoint to determine the impact on the CU consumed.

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

This instruction receives anb `u64` as part of the instruction data, which specified the number of accounts expected by the processor. The processor only asserts that the number of accounts received is the same as the `expected` number.

### Program

The program is structure in 4 different source files:

* `entrypoint.rs`: includes the entrypoint definition and "dispatches" the instruction to the corresponding processor.

* `instruction.rs`: defines the instructions available on the program and the parsing logic for the input instruction data.

* `lib.rs`: defines the modules of the program and the program ID.

* `processor.rs`: includes the processor for each instruction.

The implementation across all different entrypoint's programs are as similar as possible. In most cases, the only different is on the type import, since each entrypoint defined its own `AccountInfo` type.

## Evaluation

The evaluation is performed using [`mollusk`](https://github.com/buffalojoec/mollusk).

To run the benchmarks, you will first need to build the programs. The first time after cloning the repository, run:

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

The `ENTRYPOINT_NAME` will be one of `pinocchio`, `solana-nostd-entrypoint` or `solana-program`.

The results are written to `./target/benches/compute_units.md`. Each execution is described by 3 columns:

* `Name`: name of the benchmark; this will specify the name of the instruction and the parameters used.

* `CUs`: number of compute units consumed by the execution.

* `Delta`: the difference in compute units between latest benchmark and the previous; this will provide a quick way to assess the differences between entrypoints.