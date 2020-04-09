# WARF - WebAssembly Runtimes Fuzzing project

Goal of this project is to improve security and resilience of WebAssembly VMs/runtimes/parsers using differents fuzzing techniques.

## Quick Start

- Install system dependencies:
``` sh
# Install Rust and Cargo
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# It's better to install fuzzers using nightly compiler
rustup override set nightly

# Install honggfuzz-rs and subcommand in cargo
cargo +nightly install --force honggfuzz

# DO NOT INSTALL - FAIL FOR THE MOMENT
# Install cargo-fuzz (libfuzzer for Rust) and subcommand in cargo
# cargo +nightly install --force cargo-fuzz

# Install afl-rs and subcommand in cargo
cargo +nightly install --force afl
```

- Install WARF:
``` sh
$ git clone --depth 1 https://github.com/pventuzelo/wasm_runtimes_fuzzing
$ cd wasm_runtimes_fuzzing/warf
```

- Build & run the project:
``` sh
# Build the CLI tool
$ cargo +nightly build

# Run warf cli
$ ./target/debug/warf

warf 0.1.0
WARF - WebAssembly Runtimes Fuzzing project

USAGE:
    warf <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    continuously    Run all fuzz targets
    debug           Debug one target
    help            Prints this message or the help of the given subcommand(s)
    list-targets    List all available targets
    target          Run one target with specific fuzzer
```

- Testing:
``` sh
# Run wasmer_validate fuzzer (honggfuzz)
./target/debug/warf target wasmer_validate

------------------------[  0 days 00 hrs 00 mins 02 secs ]----------------------
  Iterations : 272,647 [272.65k]
  Mode [3/3] : Feedback Driven Mode
      Target : hfuzz_target/x86_64-unknown-linux-gnu/release/wasmer_validate
     Threads : 4, CPUs: 8, CPU%: 529% [66%/CPU]
       Speed : 171,238/sec [avg: 136,323]
     Crashes : 0 [unique: 0, blacklist: 0, verified: 0]
    Timeouts : 0 [10 sec]
 Corpus Size : 754, max: 8,192 bytes, init: 1,126 files
  Cov Update : 0 days 00 hrs 00 mins 01 secs ago
    Coverage : edge: 3,194/58,784 [5%] pc: 2 cmp: 41,653
---------------------------------- [ LOGS ] ------------------/ honggfuzz 2.0 /-
Size:77 (i,b,hw,ed,ip,cmp): 0/0/0/1/0/0, Tot:0/0/0/3159/2/41623
[...]
```

Details about the different warf subcommands [here](documentation/warf_cli_tutorial.md)


# Integration/support

Differents open-source projects (WebAssembly VMs/runtimes/parsers) will be integrated to WARF along the development.
More details [here](INTEGRATION.md)


# Roadmap

## Roadmap #1

1. List of major WebAssembly runtimes and APIs to interact with them. - [DONE](INTEGRATION.md)
2. Development of the project base (architecture and interface) - [DONE](warf/)
3. Creation of integration APIs + documentation - [DONE](warf/common/src/lib.rs) / [DONE](documentation/how_to_add_new_target.md)
4. Tutorial for project installation and testings - [DONE](README.md#quick-start)

## Roadmap #2

1.	Integration of main runtimes engines. [WIP](warf/common/src/lib.rs)
2.	CLI tool allowing execution of wasm modules through all runtimes. (using new subcommand) ?
3.	Improvement of the project (threading, runtimes perf monitoring)
4.	Development of fuzzing harness per runtimes.
5.	Dockers to install runtimes engines easily
6.	Tutorial for runtimes installation, compilation, how to run tools and unittests
7.	Unittest to verify runtimes engines work as expected

## Roadmap #3

TODO

## Roadmap #4

TODO

# Trophies

This tool helped to find the following bugs/vulnerabilities:

- TODO


# Thanks

- [Web 3 Foundation](https://web3.foundation/) for sponsoring this project.
- [Rust Fuzzing Authority](https://github.com/rust-fuzz) for Rust fuzzing tools.

# Contact

Patrick Ventuzelo - [@pat_ventuzelo](https://twitter.com/pat_ventuzelo) - Independent Security Researcher.

More details about my trainings:
* 4-days **WebAssembly security** training: [here](https://webassembly-security.com/trainings/)
* 2-days **Rustlang security** training: [here](https://webassembly-security.com/rust-security-training/)
