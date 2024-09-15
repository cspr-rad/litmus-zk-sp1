# litmus-zk-sp1

Proof of concept project.  Executes litmus verification algorithms within [SP1](https://github.com/succinctlabs/sp1).

## Requirements

- [Rust](https://rustup.rs/)
- [SP1](https://succinctlabs.github.io/sp1/getting-started/install.html)

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
