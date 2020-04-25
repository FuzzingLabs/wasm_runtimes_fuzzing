// target's fuzzing harnesses.

mod wasmi;
pub fn fuzz_wasmi_validate(data: &[u8]) {
    wasmi::fuzz_wasmi_validate(data);
}
pub fn fuzz_wasmi_instantiate(data: &[u8]) {
    wasmi::fuzz_wasmi_instantiate(data);
}

mod parity_wasm;
pub fn fuzz_parity_wasm_deserialize(data: &[u8]) {
    parity_wasm::fuzz_parity_wasm_deserialize(data);
}

mod wasmer;
pub fn fuzz_wasmer_validate(data: &[u8]) {
    wasmer::fuzz_wasmer_validate(data);
}
pub fn fuzz_wasmer_compile_clif(data: &[u8]) {
    wasmer::fuzz_wasmer_compile_clif(data);
}
pub fn fuzz_wasmer_compile_singlepass(data: &[u8]) {
    wasmer::fuzz_wasmer_compile_singlepass(data);
}
pub fn fuzz_wasmer_instantiate(data: &[u8]) {
    wasmer::fuzz_wasmer_instantiate(data);
}

mod wasmtime;
pub fn fuzz_wasmtime_validate(data: &[u8]) {
    wasmtime::fuzz_wasmtime_validate(data);
}
pub fn fuzz_wasmtime_compile(data: &[u8]) {
    wasmtime::fuzz_wasmtime_compile(data);
}
pub fn fuzz_wasmtime_all_cranelift(data: &[u8]) {
    wasmtime::fuzz_wasmtime_all_cranelift(data);
}
pub fn fuzz_wasmtime_all_lightbeam(data: &[u8]) {
    wasmtime::fuzz_wasmtime_all_lightbeam(data);
}

mod lightbeam;
pub fn fuzz_lightbeam_translate(data: &[u8]) {
    lightbeam::fuzz_lightbeam_translate(data);
}

mod wasmparser;
pub fn fuzz_wasmparser_parser(data: &[u8]) {
    wasmparser::fuzz_wasmparser_parser(data);
}
pub fn fuzz_wasmparser_validate(data: &[u8]) {
    wasmparser::fuzz_wasmparser_validate(data);
}

mod binaryen_ffi;
pub fn fuzz_binaryen_ffi(data: &[u8]) {
    binaryen_ffi::fuzz_binaryen_ffi(data);
}

mod wabt_ffi;
pub fn fuzz_wabt_wasm2wat_all_feat_ffi(data: &[u8]) {
    wabt_ffi::fuzz_wabt_wasm2wat_all_feat_ffi(data);
}
pub fn fuzz_wabt_validate_ffi(data: &[u8]) {
    wabt_ffi::fuzz_wabt_validate_ffi(data);
}

// mod cranelift
