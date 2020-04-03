# WARF - WebAssembly Runtimes Fuzzing project

Goal of this project is to improve security and resilience of WebAssembly VMs/runtimes/parsers using differents fuzzing techniques.

# Integration/support

Differents open-source projects (WebAssembly VMs/runtimes/parsers) will be integrated to WARF along the development. 


More details about current project integration/support available [here](INTEGRATION.md)

# Roadmap #1

1. List of major WebAssembly runtimes and APIs to interact with them. - [DONE](INTEGRATION.md)
2. Development of the project base (architecture and interface) - [WIP](warf/)
3. Creation of integration APIs + documentation - TODO
4. Tutorial for project installation and testings - TODO

# Roadmap #2

1.	Integration of main runtimes engines.
2.	CLI tool allowing execution of wasm modules through all runtimes.
3.	Improvement of the project (threading, runtimes perf monitoring)
4.	Development of fuzzing harness per runtimes.
5.	Dockers to install runtimes engines easily
6.	Tutorial for runtimes installation, compilation, how to run tools and unittests
7.	Unittest to verify runtimes engines work as expected

# Roadmap #3

TODO

# Roadmap #4

TODO

# Trophies

This tool helped to find the following bugs/vulnerabilities:

- TODO


# Thanks

- [Web 3 Foundation](https://web3.foundation/) for sponsoring this project.
- [Rust Fuzzing Authority](https://github.com/rust-fuzz) for Rust fuzzing tools.