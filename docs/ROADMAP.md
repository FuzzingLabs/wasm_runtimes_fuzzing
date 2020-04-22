# Roadmap

## Roadmap #1 (DONE)

1. Integration Plan / List of major WebAssembly runtimes and APIs to interact with them. - [DONE](INTEGRATION.md)
2. Project development / Development of the project base (architecture and interface) - [DONE](../warf/cli.rs)
3. APIs / Creation of integration APIs + documentation - [DONE](../warf/common/src/lib.rs) / [DONE](how_to_add_new_fuzz_target.md)
4. Delivery report / Tutorial for project installation and testings - [DONE](../README.md#quick-start)

## Roadmap #2

1.	Runtimes Integration / Integration of main runtimes engines. [WIP](../warf/common/src/lib.rs)
2.	CLI tool / Command line tool allowing execution of wasm modules through all runtimes. (using new subcommand) ?
3.	Project development / Improvement of the project (threading, runtimes perf monitoring)
4.	Project development / Development of fuzzing harness per runtimes. [WIP](../warf/common/src/lib.rs)
5.	Runtimes dockers / Dockers to install runtimes engines easily [DONE](../Dockerfile)
6.	Delivery reports / Tutorial for runtimes installation, compilation, how to run tools and unittests
7.	Unittest / Unittest to verify runtimes engines work as expected

## Roadmap #3

1.	Project development	/ Evaluation fuzzing hardness + improvement
2.	Fuzzing Implementation / Differential fuzzing implementation for wasm runtimes and parsers.
3.	Fuzzing Implementation / Grammar fuzzing implementation specific to WebAssembly module
4.	Project development / Improvement of the fuzzing (input file sharing, mutation algorithm, speed).
5.	Delivery reports / Tutorial for running fuzzers and use advanced CLI options
6.	Unittest / unit test to verify fuzzing is deterministic and reproductible

## Roadmap #4

1.	Tutorial / Runtime integration tutorial
2.	Tutorial / Utilisation tutorial
3.	Documentation / Internal architecture
4.	Documentation / Details fuzzing engines & techniques
5.	Performance testing / Improve fuzzing performances and benchmarks

## Bonus

- TODO
