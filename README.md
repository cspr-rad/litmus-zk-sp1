# litmus-zk-sp1

Proof of concept project.  Executes litmus verification algorithms within [SP1](https://github.com/succinctlabs/sp1).

## Requirements

- [Rust](https://rustup.rs/)
- [SP1](https://succinctlabs.github.io/sp1/getting-started/install.html)

## Contents

- /crates

  - Set of crates encapsulating business logic

- /elf

  - Executable Link File
  - Emitted by SP1 program compiler

- /program

  - Program to be run within SP1 ZK-VM
  - Delegates most functionality to crates

- /script

  - Driving script that executes instances of program

## Running the Project

There are two ways to run this project: compile execute a program or generate a core proof.

### Compile Program

To compile the program:

```sh
cd program
cargo prove build
```

### Execute Program

To run the program without generating a proof:

```sh
cd script
cargo run --release -- --execute
```

This will execute the program and display the output.

### Generate Program Execution Proof

To generate a core proof for your program:

```sh
cd script
cargo run --release -- --prove
```
