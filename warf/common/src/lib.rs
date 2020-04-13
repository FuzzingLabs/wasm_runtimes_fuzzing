// target's fuzzing harnesses.

/***********************************************
wasmi:
- https://github.com/paritytech/wasmi
************************************************/

/// Fuzzing `wasmi::validate_module`.
pub fn fuzz_wasmi_validate(data: &[u8]) {
    use parity_wasm::{deserialize_buffer, elements};
    use wasmi_validation::{validate_module, PlainValidator};
    let module: elements::Module = match deserialize_buffer(&data) {
        Ok(module) => module,
        _ => return,
    };
    let _ = validate_module::<PlainValidator>(&module);
}

/// Fuzzing `wasmi::ModuleInstance` with default `ImportsBuilder`.
pub fn fuzz_wasmi_instantiate(data: &[u8]) {
    use wasmi::{ImportsBuilder, Module, ModuleInstance};
    let module = match Module::from_buffer(data) {
        Ok(module) => module,
        _ => return,
    };
    let _ = ModuleInstance::new(&module, &ImportsBuilder::default());

    // TODO: add calls to instance functions like:
    // - invoke_export: https://github.com/paritytech/wasmi/blob/b67af25899874de7aac187e08e3b2a30d9bbc388/benches/src/lib.rs#L38
    // - run_start: https://github.com/paritytech/wasmi/blob/899cc32e45483fce12907f807ee9b09d837d2636/examples/interpret.rs#L36
}

/***********************************************
parity-wasm:
- https://github.com/paritytech/parity-wasm
************************************************/

/// Fuzzing `parity_wasm::deserialize_buffer`.
///
/// TODO: Will be more useful with calls to `parity_wasm::elements::Module` accessors.
/// NOTE: wasmi already use this function in `wasmi::Module::from_buffer`.
pub fn fuzz_parity_wasm_deserialize(data: &[u8]) {
    use parity_wasm::{deserialize_buffer, elements};

    let _module: std::result::Result<elements::Module, _> = deserialize_buffer(&data);
}

// TODO - parity_wasm::elements::serialize

/***********************************************
WASMER:
- https://github.com/wasmerio/wasmer
************************************************/

/// Fuzzing `wasmer::validate`
pub fn fuzz_wasmer_validate(data: &[u8]) {
    extern crate wasmer_runtime;
    let _res = wasmer_runtime::validate(&data);
}

/// Fuzzing wasmer::compile with Cranelift compiler backend
pub fn fuzz_wasmer_compile_clif(data: &[u8]) {
    use wasmer_runtime::compile;
    let _res = compile(&data);
}

/// Fuzzing `wasmer::compile` with `SinglePass` compiler backend
pub fn fuzz_wasmer_compile_singlepass(data: &[u8]) {
    use wasmer_runtime::compile_with;
    use wasmer_singlepass_backend::SinglePassCompiler;
    let _ = compile_with(&data, &SinglePassCompiler::new());
}

/// Fuzzing `wasmer::instantiate` with empty import_object
pub fn fuzz_wasmer_instantiate(data: &[u8]) {
    use wasmer_runtime::{imports, instantiate};
    let mut import_object = imports! {};
    // allow_missing_functions should prevent wasmer to reject
    // modules with imported functions
    import_object.allow_missing_functions = true;
    let _ = instantiate(data, &import_object);

    // TODO: improve or create new fuzz harness that iterate
    // over module functions and call them all
}

/*

// TODO: LLVMCompiler not available throw crates.io
pub fn TODO_fuzz_wasmer_compile_llvm(data: &[u8]) {
    extern crate wasmer_runtime;
    use std::str::FromStr;
    use wasmer_runtime::{compiler_for_backend, compile_with, Backend};
    // LLVM backend
    let backend = match Backend::from_str("llvm"){
           Ok(backend) => backend,
        _ => return,
    };
    let compiler = match compiler_for_backend(backend) {
        Some(compiler) => compiler,
        _ => return,
    };
    let _res = compile_with(&data, compiler.as_ref());
}

// TODO - wasmer_runtime::validate_and_report_errors_with_features
*/

/***********************************************
WASMTIME:
- https://github.com/bytecodealliance/wasmtime
***********************************************/

/// Fuzzing `wasmtime::validate` with default Store/Config/Engine
pub fn fuzz_wasmtime_validate(data: &[u8]) {
    use wasmtime::{Module, Store};
    let store = Store::default();
    let _module = Module::validate(&store, &data);
}

/// Fuzzing `wasmtime::Module` with default Store/Config/Engine
///
/// NOTE: wasmtime::from_binary also called wasmtime::validate.
pub fn fuzz_wasmtime_compile(data: &[u8]) {
    use wasmtime::{Module, Store};
    let store = Store::default();
    let _module = Module::from_binary(&store, &data);
}

/// Fuzzing `wasmtime::Module` with all wasm features and `Cranelift` backend.
pub fn fuzz_wasmtime_all_cranelift(data: &[u8]) {
    use wasmtime::{Config, Engine, Module, Store, Strategy};
    // Create new compilation config
    let mut config = Config::new();
    // Select Cranelift as compiler
    match config.strategy(Strategy::Cranelift) {
        Ok(o) => o,
        _ => return,
    };
    // Activate all wasm features in Config
    // https://docs.rs/wasmtime/0.15.0/wasmtime/struct.Config.html
    config
        .debug_info(true)
        .wasm_threads(true)
        .wasm_reference_types(true)
        .wasm_simd(true)
        .wasm_bulk_memory(true)
        .wasm_multi_value(true);
    let store = Store::new(&Engine::new(&config));
    // Will first validate then creates a new Module
    let _module = Module::from_binary(&store, &data);
}

/// Fuzzing `wasmtime::Module` with all wasm features and `Lightbeam` backend.
pub fn fuzz_wasmtime_all_lightbeam(data: &[u8]) {
    use wasmtime::{Config, Engine, Module, Store, Strategy};
    // Create new compilation config
    let mut config = Config::new();
    // Select Lightbeam as compiler
    match config.strategy(Strategy::Lightbeam) {
        Ok(o) => o,
        _ => return,
    };
    // Activate all wasm features in Config
    // https://docs.rs/wasmtime/0.15.0/wasmtime/struct.Config.html
    config
        .debug_info(true)
        .wasm_threads(true)
        .wasm_reference_types(true)
        .wasm_simd(true)
        .wasm_bulk_memory(true)
        .wasm_multi_value(true);
    let store = Store::new(&Engine::new(&config));
    // Will first validate then creates a new Module
    let _module = Module::from_binary(&store, &data);
}

// TODO - instantiate for Cranelift and Lightbeam.

/***********************************************
LIGHTBEAM:
- https://github.com/bytecodealliance/wasmtime/tree/master/crates/lightbeam
***********************************************/

/// Fuzzing `lightbeam::translate` using translate methods.
///
/// NOTE: lightbeam not called the same way here than in wasmtime.
/// NOTE: I'm not sure this method validate the module first.
pub fn fuzz_lightbeam_translate(data: &[u8]) {
    use lightbeam::translate;
    let _module = translate(&data);
}

/***********************************************
WASMPARSER:
- https://github.com/bytecodealliance/wasmparser
************************************************/

/// `Fuzzing wasmparser::Parser` and loop to read all module.
pub fn fuzz_wasmparser_parser(data: &[u8]) {
    use wasmparser::{Parser, ParserState, WasmDecoder};
    let mut parser = Parser::new(data);
    loop {
        match *parser.read() {
            ParserState::Error(..) | ParserState::EndWasm => break,
            _ => (),
        }
    }
}

/// Fuzzing `wasmparser::ValidatingParser` and loop to read all module.
pub fn fuzz_wasmparser_validate(data: &[u8]) {
    use wasmparser::{ParserState, ValidatingParser, WasmDecoder};
    let mut parser = ValidatingParser::new(data, None);
    loop {
        match *parser.read() {
            ParserState::Error(..) | ParserState::EndWasm => break,
            _ => (),
        }
    }
}

// TODO - wasmparser::ValidatingParserConfig

/***********************************************
BINARYEN (Rust binding using FFI)
- https://github.com/pepyakin/binaryen-rs
************************************************/

/// Fuzzing `binaryen::Module` read
///
/// NOTE: We are fuzzing binaryen over FFI.
/// TODO: Verify that binary contains coverage for C++ code of binaryen.
pub fn fuzz_binaryen_ffi(data: &[u8]) {
    use binaryen::Module;
    let _ = Module::read(&data);
}

// TODO - binaryen::Module::optimize i.e. wasm-opt

/***********************************************
WABT (Rust binding using FFI)
- https://github.com/pepyakin/wabt-rs
************************************************/

/// Fuzzing `wabt::wasm2wat_with_features` with all features enabled.
///
/// NOTE: We are fuzzing binaryen over FFI.
/// TODO: Verify if this implementation validate module first.
/// TODO: Verify that binary contains coverage for C++ code of wabt.
pub fn fuzz_wabt_wasm2wat_all_feat_ffi(data: &[u8]) {
    use wabt::{wasm2wat_with_features, Features};
    let mut features = Features::new();
    features.enable_all();
    let _ = wasm2wat_with_features(data, features);
}

/// Fuzzing `wabt::Module::{read_binary, validate}` with default features.
pub fn fuzz_wabt_validate_ffi(data: &[u8]) {
    use wabt::{Module, ReadBinaryOptions};

    // Default wasm features sets by `Module::read_binary`.
    let module = match Module::read_binary(&data, &ReadBinaryOptions::default()) {
        Ok(module) => module,
        _ => return,
    };
    // Validate the module.
    let _ = module.validate();
}

// TODO: Module::validate. - https://github.com/pepyakin/wabt-rs/blob/a8337f520b404fc09484654a4c6653ee078ac86b/src/lib.rs#L731
// TODO: `Module::read_binary` / `wasm2wat::convert`.

/***********************************************
CRANELIFT-WASM:
- https://github.com/bytecodealliance/wasmtime/tree/master/cranelift/wasm

************************************************/

// TODO - Fuzzing cranelift generation on other ISA.
/*


// Add to Cargo.toml
#cranelift-wasm = "0.62.0"
#cranelift-codegen = "0.62.0"
#target-lexicon = "0.10"


pub fn TODO_fuzz_cranelift_wasm_translate_module(data: &[u8]) {
    use cranelift_wasm::{translate_module, DummyEnvironment, ReturnMode};
    use cranelift_codegen::isa;
    use cranelift_codegen::settings::{self, Flags};
    use target_lexicon::triple;
    use std::str::FromStr;

    let flags = Flags::new(settings::builder());
    let triple = triple!("x86_64");
    let isa = isa::lookup(triple).unwrap().finish(flags.clone());
    let return_mode = ReturnMode::NormalReturns;
    let mut dummy_environ = DummyEnvironment::new(isa.frontend_config(), return_mode, false);

    let _ = translate_module(data.as_ref(), &mut dummy_environ);
}

*/

// TODO - differential fuzzing
// TODO - structural fuzzing
