# Integration

List of projects can be found below. This list is subject to changes in the future.

Regarding APIs, we will focus first on apis in charge of the following operations:
- [YES] parsing/deserialization.
- [YES] validation.
- [YES] instantiation.
- [YES] execution.
- [MAYBE] serialization.

# Projects integrated

Those projects **are already** supported/integrated.

- TODO


# Projects to be integrated.

Those projects **will be** supported/integrated.

## wasmi (Rust)

Wasm interpreter in Rust: [github](https://github.com/paritytech/wasmi) / [documentation](https://paritytech.github.io/wasmi/wasmi/index.html)

- APIs:
	- `wasmi::Module::from_buffer`: Load, validate and prepare a `parity_wasm`'s `Module`. - [impl](https://github.com/paritytech/wasmi/blob/b67af25899874de7aac187e08e3b2a30d9bbc388/src/lib.rs#L426)
	- `validate_module`: A module validator function - [impl](https://github.com/paritytech/wasmi/blob/e8d5fb6c84edee2b43e99113cfdc19951520c29a/validation/src/lib.rs#L131)
	- `wasmi::ModuleInstance::new`: Runtime representation of a `wasmi::Module` - [example](https://github.com/paritytech/wasmi/blob/master/examples/interpret.rs#L34)

- Examples:
	- [tests](https://github.com/paritytech/wasmi/blob/899cc32e45483fce12907f807ee9b09d837d2636/src/tests/wasm.rs)
	- [examples](https://github.com/paritytech/wasmi/tree/master/examples)
	- [fuzzing](https://github.com/paritytech/wasmi/blob/master/hfuzz/src/main.rs)
	- [fuzzing](https://github.com/paritytech/wasmi/tree/master/fuzz/fuzz_targets)

## wasmtime (Rust)

Standalone JIT-style runtime for WebAssembly: [github](https://github.com/bytecodealliance/wasmtime) / [guide](https://bytecodealliance.github.io/wasmtime/)


- backends:
	- [cranelift](https://github.com/bytecodealliance/wasmtime/tree/master/cranelift)
	- [lightbeam](https://github.com/bytecodealliance/wasmtime/tree/master/crates/lightbeam)

- APIs:
	- `oracles::compile`: Generic way to compile wasm module bytes - [example](https://github.com/bytecodealliance/wasmtime/blob/master/fuzz/fuzz_targets/compile.rs)
	- `oracles::instantiate`: Generic way to compile and instantiate wasm module bytes - [example](https://github.com/bytecodealliance/wasmtime/blob/master/fuzz/fuzz_targets/instantiate.rs) / [other](https://github.com/bytecodealliance/wasmtime/blob/b3ac71842183ca99cfa8a2d04e9a7ac5a2cee50d/crates/fuzzing/tests/regressions.rs)
	- `lightbeam::translate`: Translate wasm module bytes to `TranslatedModule` Struct [impl](https://github.com/bytecodealliance/wasmtime/blob/master/crates/lightbeam/src/module.rs#L503)

- Examples:
	- [tests](https://github.com/bytecodealliance/wasmtime/tree/master/tests)
	- [examples](https://github.com/bytecodealliance/wasmtime/tree/master/examples)
	- [fuzzing](https://github.com/bytecodealliance/wasmtime/tree/master/fuzz)

## wasmer (Rust)

Standalone WebAssembly runtime: [github](https://github.com/wasmerio/wasmer) / [documentation](https://docs.wasmer.io/)

- backends:
	- [singlepass](https://github.com/wasmerio/wasmer/tree/master/lib/singlepass-backend)
	- [cranelift](https://github.com/wasmerio/wasmer/tree/master/lib/clif-backend)
	- [LLVM](https://github.com/wasmerio/wasmer/tree/master/lib/llvm-backend)

- APIs:
	- `wasmer_runtime::validate`: Perform validation of the wasm module - [doc](https://docs.rs/wasmer-runtime-core/0.16.2/src/wasmer_runtime_core/lib.rs.html#140-142)
	- `wasmer_runtime::validate_and_report_errors_with_features`: Perform validation with a Features - doc[https://docs.rs/wasmer-runtime-core/0.7.0/wasmer_runtime_core/fn.validate_and_report_errors_with_features.html]
	- `wasmer_runtime::{compile, compile_with}`: Compile WebAssembly binary code into a Module, backends can be specified here - [doc](https://docs.rs/wasmer-runtime/0.16.2/wasmer_runtime/fn.compile.html)
	- `wasmer_runtime::instantiate`: Compile and instantiate wasm code - [doc](https://docs.rs/wasmer-runtime/0.16.2/wasmer_runtime/fn.instantiate.html)

- Examples:
	- [examples](https://github.com/wasmerio/wasmer/tree/master/examples)
	- [tests](https://github.com/wasmerio/wasmer/tree/master/lib/spectests)
	- [fuzzing](https://github.com/wasmerio/wasmer/tree/master/fuzz)
	- [fuzzing](https://github.com/wasmerio/wasm-fuzz)

## parity-wasm (Rust)

WebAssembly serialization/deserialization in rust: [github](https://github.com/paritytech/parity-wasm) / [documentation](https://docs.rs/parity-wasm/0.41.0/parity_wasm/)

- APIs:
	- `parity_wasm::deserialize_file`: module parsing - [test](https://github.com/paritytech/parity-wasm/blob/master/src/elements/module.rs#L650-L656)
	- `parity_wasm::Module` struct: WebAssembly module [impl](https://github.com/paritytech/parity-wasm/blob/master/src/elements/module.rs#L48)
	- `parity_wasm::serialize_to_file`: will not be supported for the moment.

- Examples:
	- [tests](https://github.com/paritytech/parity-wasm/blob/master/src/elements/module.rs#L650-L656)
	- [examples](https://github.com/paritytech/parity-wasm/tree/master/examples)
	- [fuzzing](https://github.com/paritytech/parity-wasm/blob/master/fuzz/fuzz_targets/deserialize.rs)

## wasmparser (Rust)

A simple event-driven library for parsing WebAssembly binary files: [github](https://github.com/bytecodealliance/wasmparser) / [documentation](https://docs.rs/wasmparser/0.51.4/wasmparser/)

- APIs:
	- `wasmparser::Parser`: Event-driven parser of WebAssembly binary - [impl](https://github.com/bytecodealliance/wasmparser/blob/master/src/parser.rs#L212)
	- `wasmparser::ValidatingParser`: validate module depending of provided config - [impl](https://github.com/bytecodealliance/wasmparser/blob/master/src/validator.rs#L157)
	- `wasmparser::ValidatingParserConfig`: validate module depending of provided config - [impl](https://github.com/bytecodealliance/wasmparser/blob/master/src/validator.rs#L89)

- Examples:
	- [tests](https://github.com/bytecodealliance/wasmparser/tree/master/tests)
	- [examples](https://github.com/bytecodealliance/wasmparser/tree/master/examples)
	- [fuzzing](https://github.com/bytecodealliance/wasmparser/tree/master/fuzz/fuzz_targets)

## binaryen (C++/Rust)

Compiler infrastructure and toolchain: [github](https://github.com/WebAssembly/binaryen) / [rust bindings](https://github.com/pepyakin/binaryen-rs)

- Rust APIs:
	- `binaryen::Module::read`: Deserialize a module from binary form. - [impl](https://github.com/pepyakin/binaryen-rs/blob/abe2babb2d1d8e88a5f2aa47fb6e24393e19e8c0/src/lib.rs#L64)
	- `binaryen::Module::optimize`: Run the standard optimization passes on the module. - [impl](https://github.com/pepyakin/binaryen-rs/blob/abe2babb2d1d8e88a5f2aa47fb6e24393e19e8c0/src/lib.rs#L81)
	- `Binaryen` interpreter: Simple WebAssembly interpreter - [code](https://github.com/WebAssembly/binaryen/blob/master/src/wasm-interpreter.h)

- Rust examples:
	- [examples](https://github.com/pepyakin/binaryen-rs/tree/abe2babb2d1d8e88a5f2aa47fb6e24393e19e8c0/examples)

## wabt

- [wabt](https://github.com/WebAssembly/wabt) - The WebAssembly Binary Toolkit - [rust bindings](https://github.com/pepyakin/wabt-rs)

- Rust Apis:
	- `Module`: WebAssembly module. (take a `Features` struct - possible to active `enable_all`(https://github.com/pepyakin/wabt-rs/blob/master/src/lib.rs#L182))
	- `Module::read_binary` / `wasm2wat::convert`: useful if we validate the module first since `read_binary doesn't do any validation`.
	- `wasm2wat`: Disassemble wasm binary to wasm text format. - [doc](https://docs.rs/wabt/0.9.2/wabt/fn.wasm2wat.html)
	- `wasm2wat_with_features`: Disassemble wasm binary to wasm text format with the given features. - [doc](https://docs.rs/wabt/0.9.2/wabt/fn.wasm2wat_with_features.html)
	- `Module::validate`: Validate the module. - [impl](https://github.com/pepyakin/wabt-rs/blob/master/src/lib.rs#L731)
	- `wat2wasm`: Translate wasm text source to wasm binary format. - NOT A PRIORITY TO IMPLEMENT - [doc](https://docs.rs/wabt/0.9.2/wabt/fn.wat2wasm.html)
	- `wat2wasm_with_features`: Translate wasm text source to wasm binary format with the given features. - NOT A PRIORITY TO IMPLEMENT - [doc](https://docs.rs/wabt/0.9.2/wabt/fn.wat2wasm_with_features.html)


- Rust examples:
	- [examples](https://github.com/pepyakin/wabt-rs/blob/a8337f520b404fc09484654a4c6653ee078ac86b/src/lib.rs#L1111)

# Projects potentially integrated.

Those projects **will be potentially** supported/integrated in the future.

- [wasm3](https://github.com/wasm3/wasm3) - high performance WebAssembly interpreter written in C.
- [WAVM](https://github.com/WAVM/WAVM) - WebAssembly Virtual Machine in C++.
- [webassemblyjs](https://github.com/xtuc/webassemblyjs) - Toolchain for WebAssembly in JavaScript.
