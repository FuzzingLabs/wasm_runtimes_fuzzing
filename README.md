# WARF - WebAssembly Runtimes Fuzzing project

Goal of this project is to improve security and resilience of WebAssembly VMs/runtimes/parsers using different fuzzing techniques.

## Quick Start (using docker)

- Clone the project
``` sh
# Install WARF
$ git clone --depth 1 https://github.com/pventuzelo/wasm_runtimes_fuzzing
$ cd wasm_runtimes_fuzzing/warf
```

Build warf with docker:
``` sh
# Build warf docker
$ make docker
# Optional: Create an alias
$ alias warf="docker run -it -v `pwd`/workspace:/warf/workspace warf"
# ==> workspace folder is shared between your host and docker container.
```
NOTE: If you are on running on `Ubuntu`, installation without docker can be found [here](docs/INSTALL.md).


- Run warf cli:
``` sh
$ warf help

WARF - WebAssembly Runtimes Fuzzing project
USAGE:
    warf <SUBCOMMAND>
FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
SUBCOMMANDS:
    benchmark-all    Run WebAssembly module on all targets with benchmark
    build            Build all targets for this specific fuzzer
    continuously     Run all fuzz targets
    debug            Debug one target
    execute-all      Run WebAssembly module on all targets
    help             Prints this message or the help of the given subcommand(s)
    list             List all available targets
    target           Run one target with specific fuzzer
```
NOTE: Details about the different warf subcommands [here](docs/WARF_SUBCOMMANDS.md).

- List available fuzzing targets:
``` sh
$ warf list

wasmi_validate
wasmi_instantiate
parity_wasm_deserialize
[...]
binaryen_ffi
wabt_wasm2wat_all_feat_ffi
wabt_validate_ffi
```

- Run fuzzing on a target:
``` sh
$ warf target wasmer_validate

[...]

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

# Tests

Tests are documented inside the `Makefile`:
``` sh
$ make help
Management commands for warf

Usage:
    make build                            Compile the project locally.
    make docker                           Build a docker image for this project.
    make corpora                          TODO

    make fmt                              Run Rust fmt.
    make clean                            Clean only warf binary.
    make clean-all                        Clean all (warf && compiled fuzz target harnesses).

    make test                                         Simple test to check warf and execute_all is working.
    make test-bench                                   Simple benchmark using execute_all.
    make test-debug                                   Test running a simple wasm to a debugging tool.
    make test-{libfuzzer, honggfuzz, afl}             Test one fuzzing hardness over choosen fuzzer.
    make test-continuously-{libfuzzer, hfuzz, afl}    Test all fuzzing hardness over choosen fuzzer.
    make test-all                                     Test all fuzzing hardness over all fuzzers.

```

If you are using docker, try:
``` sh
make docker-test
make docker-test-all
```

# Future of the project

Differents open-source projects (WebAssembly VMs/runtimes/parsers) will be integrated to WARF along the development:
- Integration details [here](docs/INTEGRATION.md).
- Global roadmap [here](docs/ROADMAP.md).

# Trophies

This tool helped to find the following bugs/vulnerabilities (crashing files are inside `trophies` folder):
- wasmer/wasmer_clif_fork_wasm: [index out of bounds panic](https://github.com/wasmerio/wasmer/issues/1372)
- binaryen: [segfault / out-of-bounds read in `WasmBinaryBuilder::readImports`](https://github.com/WebAssembly/binaryen/issues/2751) - **FIXED**
- wabt: [SIGABRT due to std::bad_alloc exception (resizing wasm br_table)](https://github.com/WebAssembly/wabt/issues/1386) - **FIXED**
- wasmtime: [assertion failed in wasmtime_debug::transform::simulate::generate_simulated_dwarf](https://github.com/bytecodealliance/wasmtime/issues/1506) - **FIXED**
- wasmtime: [assertion failed or unimplemented panic when table type is not anyref](https://github.com/bytecodealliance/wasmtime/issues/1601) - **FIXED**
- wabt: [[wasm2wat] Assertion failure in `BinaryReaderIR::OnCallIndirectExpr`](https://github.com/WebAssembly/wabt/issues/1413) - **FIXED**
- wabt: [[wasm2wat] Assertion failure in `BinaryReaderIR::OnReturnCallIndirectExpr`](https://github.com/WebAssembly/wabt/issues/1414) - **FIXED**
- wabt: [Incorrect validation of module with malformed alignment by wabt](https://github.com/WebAssembly/wabt/issues/1453) - **FIXED**
- wabt: [[wasm2wat] Incorrect rejection of valid module](https://github.com/WebAssembly/wabt/issues/1455)
- wain: [unwrap panic while parsing invalid wasm module](https://github.com/rhysd/wain/issues/29) - **FIXED**
- wain: [memory allocation failed error during parsing](https://github.com/rhysd/wain/issues/30) - **FIXED**
- wasm3: [segfault / assertion failed in `GetStackTopIndex`](https://github.com/wasm3/wasm3/issues/151) - **FIXED**
- wasm3: [segfault / null pointer dereference in `GetFunctionNumReturns`](https://github.com/wasm3/wasm3/issues/152) - **FIXED**
- wasm3: [heap buffer overflow in `ParseSection_Export`](https://github.com/wasm3/wasm3/issues/153) - **FIXED**
- wasm3: [SIGILL in `Compile_BlockStatements`](https://github.com/wasm3/wasm3/issues/154) - **FIXED**
- wain: ["index out of bounds" in wain validate](https://github.com/rhysd/wain/issues/39) - **FIXED**
- wasmprinter: [Resources exhaustion (CPU/MEM) using `wasmprinter::print_bytes()`](https://github.com/bytecodealliance/wasm-tools/issues/52) - **FIXED**
- wasm3: [heap-use-after-free in `ReadLebUnsigned`](https://github.com/wasm3/wasm3/issues/156) - **FIXED**
- wasm3: [global-buffer-overflow in `Compile_BlockStatements`](https://github.com/wasm3/wasm3/issues/157) - **FIXED**
- wasm3: [out of bound read in `Read_f64`](https://github.com/wasm3/wasm3/issues/158) - **FIXED**
- wasm3: [heap-buffer-overflow in `Compile_BlockStatements` (line 2169)](https://github.com/wasm3/wasm3/issues/159) - **FIXED**

# Thanks

- [Web 3 Foundation](https://web3.foundation/) for sponsoring this project.
- [Rust Fuzzing Authority](https://github.com/rust-fuzz) for Rust fuzzing tools.


# Trainings & Contact

Patrick Ventuzelo - [@pat_ventuzelo](https://twitter.com/pat_ventuzelo)
* Independent Security Researcher / Trainer.
* FREE online courses: [here](https://academy.fuzzinglabs.com/)
