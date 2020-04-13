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


# Future of the project

Differents open-source projects (WebAssembly VMs/runtimes/parsers) will be integrated to WARF along the development.
More details [here](documentation/INTEGRATION.md)


Global roadmap [here](documentation/ROADMAP.md)


# Trophies

This tool helped to find the following bugs/vulnerabilities (crashing files are inside `trophies` folder):

- [wasmer/wasmer_clif_fork_wasm: index out of bounds panic](https://github.com/wasmerio/wasmer/issues/1372)
- [binaryen: segfault / out-of-bounds read in WasmBinaryBuilder::readImports](https://github.com/WebAssembly/binaryen/issues/2751)
- [wabt: SIGABRT due to std::bad_alloc exception (resizing wasm br_table)](https://github.com/WebAssembly/wabt/issues/1386)




# Thanks

- [Web 3 Foundation](https://web3.foundation/) for sponsoring this project.
- [Rust Fuzzing Authority](https://github.com/rust-fuzz) for Rust fuzzing tools.

# Contact

Patrick Ventuzelo - [@pat_ventuzelo](https://twitter.com/pat_ventuzelo) - Independent Security Researcher.

Consulting & trainings:
* 4-days **WebAssembly security** training: [here](https://webassembly-security.com/trainings/)
* 2-days **Rustlang security** training: [here](https://webassembly-security.com/rust-security-training/)
