# Roadmap

## Roadmap #1 (DONE)

1. Integration Plan / List of major WebAssembly runtimes and APIs to interact with them. - [DONE](INTEGRATION.md)
2. Project development / Development of the project base (architecture and interface) - [DONE](../warf/src/main.rs)
3. APIs / Creation of integration APIs + documentation - [DONE](../warf/targets/src/lib.rs) / [DONE](how_to_add_new_fuzz_target.md)
4. Delivery report / Tutorial for project installation and testings - [DONE](../README.md#quick-start)

## Roadmap #2 (DONE)

1.	Runtimes Integration / Integration of main runtimes engines. - [DONE](../warf/targets/src/lib.rs)
2.	CLI tool / Command line tool allowing execution of wasm modules through all runtimes. - [DONE - ./warf execute-all, make test](../warf/src/main.rs)
3.	Project development / Improvement of the project (threading, runtimes perf monitoring) - [DONE - benchmark subcommand, make test-bench](../warf/src/main.rs)
4.	Project development / Development of fuzzing harness per runtimes. - [DONE](../warf/targets/src/)
5.	Runtimes dockers / Dockers to install runtimes engines easily - [DONE](../Dockerfile)
6.	Delivery reports / Tutorial for runtimes installation, compilation, how to run tools and unittests - [DONE](../README.md) / [DONE](WARF_SUBCOMMANDS.md) / [DONE](../docs/)
7.	Unittest / Unittest to verify runtimes engines work as expected - [DONE - make test\*](../warf/Makefile)

## Roadmap #3 (DONE)

1.	Project development	/ Evaluation fuzzing hardness + improvement - [DONE - Add more fuzzing targets ([wasmprinter](../warf/targets/src/wasmprinter.rs), [wain](../warf/targets/src/wain.rs), [wat](../warf/targets/src/wat.rs), [wast](../warf/targets/src/wast.rs), [wasm3](../warf/targets/src/wasm3.rs))]
2.	Fuzzing Implementation / Differential fuzzing implementation for wasm runtimes and parsers. - [DONE](../warf/targets/src/lib.rs)
3.	Fuzzing Implementation / Grammar fuzzing implementation specific to WebAssembly module - [DONE - dictionary option](../warf/dictionary)
4.	Project development / Improvement of the fuzzing (input file sharing, mutation algorithm, speed). - [DONE - sharing folder for samples](../warf/src/targets.rs)
5.	Delivery reports / Tutorial for running fuzzers and use advanced CLI options - [DONE](warf_advanced_options.md)
6.	Unittest / unit test to verify fuzzing is deterministic and reproductible - [DONE - seed option make fuzzing reproduc mis make](../warf/src/main.rs)

## Roadmap #4 (DONE)

1.	Tutorial / Runtime integration tutorial - [DONE - how_to_add_new_fuzz_target.md](how_to_add_new_fuzz_target.md)
2.	Tutorial / Utilisation tutorial - DONE [WARF_SUBCOMMANDS.md](WARF_SUBCOMMANDS.md), [warf_advanced_options.md](warf_advanced_options.md)
3.	Documentation / Internal architecture - [DONE - Internals.md](Internals.md#global-functionnement)
4.	Documentation / Details fuzzing engines & techniques - [DONE - Internals.md](Internals.md#fuzzing-engines)
5.	Performance testing / Improve fuzzing performances and benchmarks - [DONE - Leveraging Rust borrowing to improve fuzzing performances](https://github.com/pventuzelo/wasm_runtimes_fuzzing/commit/7b47782f01187571446f2f2d376a1d6183ee68ff)
