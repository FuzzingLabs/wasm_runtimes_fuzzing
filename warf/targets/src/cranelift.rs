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