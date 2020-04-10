// target's fuzzing functions

/***********************************************
parity-wasm:
- https://github.com/paritytech/parity-wasm
************************************************/

pub fn fuzz_parity_wasm_deserialize(data: &[u8]) {
    extern crate parity_wasm;
    let _module: std::result::Result<parity_wasm::elements::Module, _> =
        parity_wasm::deserialize_buffer(&data);
}

/***********************************************
WASMER:
- https://github.com/wasmerio/wasmer
************************************************/

///  Fuzzing wasmer::validate
pub fn fuzz_wasmer_validate(data: &[u8]) {
    extern crate wasmer_runtime;
    let _res = wasmer_runtime::validate(&data);
}

///  Fuzzing wasmer::compile with Cranelift compiler backend
pub fn fuzz_wasmer_compile_clif(data: &[u8]) {
    use wasmer_runtime::compile;
    let _res = compile(&data);
}

///  Fuzzing wasmer::compile with SinglePass compiler backend
pub fn fuzz_wasmer_compile_singlepass(data: &[u8]) {
    use wasmer_runtime::compile_with;
    use wasmer_singlepass_backend::SinglePassCompiler;
    let _ = compile_with(&data, &SinglePassCompiler::new());
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
// TODO - wasmer_runtime::instantiate
*/

/***********************************************
WASMTIME:
- https://github.com/bytecodealliance/wasmtime
***********************************************

// TODO: need to be git clone
// prefered way is to use crates.io

pub fn TODO_fuzz_wasmtime_compile(data: &[u8]) {
    use wasmtime::Strategy;
    use wasmtime_fuzzing::oracles;
    // only Cranelift backend
    oracles::compile(data, Strategy::Cranelift);
    // TODO: add Lightbeam backend
}

pub fn TODO_fuzz_wasmtime_instantiate(data: &[u8]) {
    use wasmtime::Strategy;
    use wasmtime_fuzzing::oracles;
    // only Cranelift backend
    oracles::instantiate(data, Strategy::Cranelift);
    // TODO: add Lightbeam backend
}
*/

/***********************************************
WASMPARSER:
- https://github.com/bytecodealliance/wasmparser
************************************************/

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
CRANELIFT-WASM:
- https://github.com/bytecodealliance/wasmtime/tree/master/cranelift/wasm

FUZZING NOT USEFUL:
`translate_module` is supposed to only process valid module.
************************************************/

/*
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
