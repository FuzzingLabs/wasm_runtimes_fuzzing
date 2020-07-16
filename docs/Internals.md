# Global functionnement

This tool dynamicaly create fuzzing harnesses depending of the specified fuzzing engine (honggfuzz, libfuzzer, afl). 
In short, when warf is executed, it will copy a fuzzing or debug template, replace some code inside to specify the fuzzing target library and finally run the fuzzer. 

If you are looking to add new targets, take a look at [how_to_add_new_fuzz_target.md](how_to_add_new_fuzz_target.md).

# Architecture of the project

 ``` sh
$ tree -L 1
.
├── Cargo.lock
├── Cargo.toml
├── debug             # debugging subcommand template
├── dictionary        # fuzzing dictionnaries
├── Dockerfile        # Docker file
├── execute_all       # execute-all subcommand template
├── fuzzers           # fuzzing engines templates
├── Makefile
├── src               # source code of warf
├── targets           # fuzzing targets source code
├── warf
└── workspace         # workspace shared between host and docker VM, fuzzing templates/inputs/logs/crashes are there.
```

# Templates

Imagine, you are running the following command: `./warf target wasmparser_parser --fuzzer Honggfuzz`

warf will do the following actions internally:
- Copy `warf/fuzzers/rust-honggfuzz` folder inside the `workspace`
- Copy the Honggfuzz fuzzing template `template.rs` into a new one like `workspace/hfuzz/src/bin/wasmparser_parser.rs`
- Replace `###TARGET###` inside this template by the name of the target we choose (`fuzz_wasmparser_parser`)
- Compile the fuzzing harness and run honggfuzz.

# Fuzzing engines

Warf support 3 different Rust fuzzing engines:
- honggfuzz-rs
- cargo-fuzz (libfuzzer)
- afl.rs (AFL++)

## honggfuzz-rs

*Honggfuzz is security oriented, feedback-driven, evolutionary, easy-to-use fuzzer with interesting analysis options* - [source](https://github.com/google/honggfuzz)

Honggfuzz for Rust is available here: [honggfuzz-rs](https://github.com/rust-fuzz/honggfuzz-rs) / [Documentation](https://docs.rs/honggfuzz/0.5.45/honggfuzz/) and can be used with:
- Rust: stable, beta, nightly.
- Sanitizer: none, address, thread, leak.

Full compatibility list [here](https://github.com/rust-fuzz/honggfuzz-rs#compatibility)

## cargo-fuzz (libfuzzer)

Command-line wrapper for using libFuzzer. Easy to use, no need to recompile LLVM!

Cargo-fuzz repository: [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz).

`cargo-fuzz` is documented in the [Rust Fuzz Book](https://rust-fuzz.github.io/book/cargo-fuzz.html).

## afl.rs

American fuzzy lop (AFL) is a popular, effective, and modern fuzz testing tool. `afl.rs` allows one to run AFL on code written in the Rust programming language.

`afl-rs` can be found [here](https://github.com/rust-fuzz/afl.rs) and some documention are in the [Rust Fuzz Book](https://rust-fuzz.github.io/book/afl.html).

# Fuzzing techniques

All previous fuzzing engines used those differents fuzzing techniques to find bugs:
- Coverage-guided fuzzing: Target if compiled with some instrumentation piece of code used to monitor if some new code path has been reached during fuzzing session.
- Mutation-based fuzzing: Input corpora is used and mutated/modified (using various algorithm) before being provided to the target.
- Grammar-based fuzzing: Fuzzing dictionnaries will help the fuzzing engines to generate valid mutated samples.
- Differential fuzzing: Result of different fuzzing targets are compared in order to find logic bugs (like improper validation or rejection of an input module)
