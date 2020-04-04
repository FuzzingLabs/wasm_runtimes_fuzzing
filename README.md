# WARF - WebAssembly Runtimes Fuzzing project

Goal of this project is to improve security and resilience of WebAssembly VMs/runtimes/parsers using differents fuzzing techniques.

## Integration/support

Differents open-source projects (WebAssembly VMs/runtimes/parsers) will be integrated to WARF along the development. 


More details about current project integration/support available [here](INTEGRATION.md)


## Quick Start

- Install system dependencies:
``` sh
# Install Rust and Cargo
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install honggfuzz-rs and subcommand in cargo
cargo +nightly install --force honggfuzz

# Install cargo-fuzz (libfuzzer for Rust) and subcommand in cargo
cargo +nightly install --force cargo-fuzz

# Install afl-rs and subcommand in cargo
cargo +nightly install --force afl
```

- Install WARF:
``` sh
$ git clone --depth 1 https://github.com/pventuzelo/wasm_runtimes_fuzzing
$ cd wasm_runtimes_fuzzing
```

- Build & run the project:
``` sh
# we are using nightly compiler
cargo +nightly build
./target/debug/warf

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

# Roadmap

## Roadmap #1

1. List of major WebAssembly runtimes and APIs to interact with them. - [DONE](INTEGRATION.md)
2. Development of the project base (architecture and interface) - [DONE](warf/)
3. Creation of integration APIs + documentation - [DONE](warf/common/src/lib.rs) / [DONE](documentation/how_to_add_new_target.md)
4. Tutorial for project installation and testings - [WIP](README.md#quick-start)

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
