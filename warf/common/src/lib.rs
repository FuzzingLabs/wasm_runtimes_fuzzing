// target's fuzzing functions

#[inline(always)]
pub fn fuzz_parity_wasm_deserialize(data: &[u8]) {
    extern crate parity_wasm;
    let _module: std::result::Result<parity_wasm::elements::Module, _> =
        parity_wasm::deserialize_buffer(&data);
}

#[inline(always)]
pub fn fuzz_wasmer_validate(data: &[u8]) {
    extern crate wasmer_runtime;
    let _res = wasmer_runtime::validate(&data);
}

// TODO - wasmer_runtime::validate_and_report_errors_with_features
// TODO - wasmer_runtime::compile
// TODO - wasmer_runtime::compile_with (all backends)
// TODO - wasmer_runtime::instantiate

// TODO - differential fuzzing

// TODO - structural fuzzing
