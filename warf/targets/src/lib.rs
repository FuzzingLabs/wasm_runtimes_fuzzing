mod wasmi;
// fuzzing harnesses
pub fn fuzz_wasmi_validate(data: &[u8]) {
    let _ = wasmi::wasmi_validate(data);
}
pub fn fuzz_wasmi_instantiate(data: &[u8]) {
    let _ = wasmi::wasmi_instantiate(data);
}
// debug target
pub fn debug_wasmi_validate(data: &[u8]) -> bool {
    wasmi::wasmi_validate(data)
}
pub fn debug_wasmi_instantiate(data: &[u8]) -> bool {
    wasmi::wasmi_instantiate(data)
}

mod parity_wasm;
// fuzzing harnesses
pub fn fuzz_parity_wasm_deserialize(data: &[u8]) {
    let _ = parity_wasm::parity_wasm_deserialize(data);
}
// debug target
pub fn debug_parity_wasm_deserialize(data: &[u8]) -> bool {
    parity_wasm::parity_wasm_deserialize(data)
}

mod wasmer;
// fuzzing harnesses
pub fn fuzz_wasmer_validate(data: &[u8]) {
    let _ = wasmer::fuzz_wasmer_validate(data);
}
pub fn fuzz_wasmer_compile_clif(data: &[u8]) {
    let _ = wasmer::fuzz_wasmer_compile_clif(data);
}
pub fn fuzz_wasmer_compile_singlepass(data: &[u8]) {
    let _ = wasmer::fuzz_wasmer_compile_singlepass(data);
}
pub fn fuzz_wasmer_instantiate(data: &[u8]) {
    let _ = wasmer::fuzz_wasmer_instantiate(data);
}
// debug target
pub fn debug_wasmer_validate(data: &[u8]) -> bool {
    wasmer::fuzz_wasmer_validate(data)
}
pub fn debug_wasmer_compile_clif(data: &[u8]) -> bool {
    wasmer::fuzz_wasmer_compile_clif(data)
}
pub fn debug_wasmer_compile_singlepass(data: &[u8]) -> bool {
    wasmer::fuzz_wasmer_compile_singlepass(data)
}
pub fn debug_wasmer_instantiate(data: &[u8]) -> bool {
    wasmer::fuzz_wasmer_instantiate(data)
}

mod wasmtime;
// fuzzing harnesses
pub fn fuzz_wasmtime_validate(data: &[u8]) {
    let _ = wasmtime::fuzz_wasmtime_validate(data);
}
pub fn fuzz_wasmtime_validate_all_feat(data: &[u8]) {
    let _ = wasmtime::fuzz_wasmtime_validate_all_feat(data);
}
pub fn fuzz_wasmtime_compile(data: &[u8]) {
    let _ = wasmtime::fuzz_wasmtime_compile(data);
}
pub fn fuzz_wasmtime_compile_all_cranelift(data: &[u8]) {
    let _ = wasmtime::fuzz_wasmtime_compile_all_cranelift(data);
}
pub fn fuzz_wasmtime_compile_all_lightbeam(data: &[u8]) {
    let _ = wasmtime::fuzz_wasmtime_compile_all_lightbeam(data);
}
pub fn fuzz_wasmtime_instantiate_all_cranelift(data: &[u8]) {
    let _ = wasmtime::fuzz_wasmtime_instantiate_all_cranelift(data);
}
pub fn fuzz_wasmtime_instantiate_all_lightbeam(data: &[u8]) {
    let _ = wasmtime::fuzz_wasmtime_instantiate_all_lightbeam(data);
}
// debug target
pub fn debug_wasmtime_validate(data: &[u8]) -> bool {
    wasmtime::fuzz_wasmtime_validate(data)
}
pub fn debug_wasmtime_validate_all_feat(data: &[u8]) -> bool {
    wasmtime::fuzz_wasmtime_validate_all_feat(data)
}
pub fn debug_wasmtime_compile(data: &[u8]) -> bool {
    wasmtime::fuzz_wasmtime_compile(data)
}
pub fn debug_wasmtime_compile_all_cranelift(data: &[u8]) -> bool {
    wasmtime::fuzz_wasmtime_compile_all_cranelift(data)
}
pub fn debug_wasmtime_compile_all_lightbeam(data: &[u8]) -> bool {
    wasmtime::fuzz_wasmtime_compile_all_lightbeam(data)
}
pub fn debug_wasmtime_instantiate_all_cranelift(data: &[u8]) -> bool {
    wasmtime::fuzz_wasmtime_instantiate_all_cranelift(data)
}
pub fn debug_wasmtime_instantiate_all_lightbeam(data: &[u8]) -> bool {
    wasmtime::fuzz_wasmtime_instantiate_all_lightbeam(data)
}

mod lightbeam;
// fuzzing harnesses
pub fn fuzz_lightbeam_translate(data: &[u8]) {
    let _ = lightbeam::fuzz_lightbeam_translate(data);
}
// debug target
pub fn debug_lightbeam_translate(data: &[u8]) -> bool {
    lightbeam::fuzz_lightbeam_translate(data)
}

mod wasmparser;
// fuzzing harnesses
pub fn fuzz_wasmparser_parser(data: &[u8]) {
    let _ = wasmparser::fuzz_wasmparser_parser(data);
}
pub fn fuzz_wasmparser_validate(data: &[u8]) {
    let _ = wasmparser::fuzz_wasmparser_validate(data);
}
pub fn fuzz_wasmparser_validate_all_feat(data: &[u8]) {
    let _ = wasmparser::fuzz_wasmparser_validate_all_feat(data);
}
// debug target
pub fn debug_wasmparser_parser(data: &[u8]) -> bool {
    wasmparser::fuzz_wasmparser_parser(data)
}
pub fn debug_wasmparser_validate(data: &[u8]) -> bool {
    wasmparser::fuzz_wasmparser_validate(data)
}
pub fn debug_wasmparser_validate_all_feat(data: &[u8]) -> bool {
    wasmparser::fuzz_wasmparser_validate_all_feat(data)
}

mod binaryen_ffi;
// fuzzing harnesses
pub fn fuzz_binaryen_ffi(data: &[u8]) {
    let _ = binaryen_ffi::fuzz_binaryen_ffi(data);
}
pub fn fuzz_binaryen_optimize_ffi(data: &[u8]) {
    let _ = binaryen_ffi::fuzz_binaryen_optimize_ffi(data);
}
// debug target
pub fn debug_binaryen_ffi(data: &[u8]) -> bool {
    binaryen_ffi::fuzz_binaryen_ffi(data)
}
pub fn debug_binaryen_optimize_ffi(data: &[u8]) -> bool {
    binaryen_ffi::fuzz_binaryen_optimize_ffi(data)
}

mod wabt_ffi;

pub fn fuzz_wabt_wasm2wat_all_feat_ffi(data: &[u8]) {
    let _ = wabt_ffi::fuzz_wabt_wasm2wat_all_feat_ffi(data);
}
pub fn fuzz_wabt_validate_ffi(data: &[u8]) {
    let _ = wabt_ffi::fuzz_wabt_validate_ffi(data);
}
// debug target
pub fn debug_wabt_wasm2wat_all_feat_ffi(data: &[u8]) -> bool {
    wabt_ffi::fuzz_wabt_wasm2wat_all_feat_ffi(data)
}
pub fn debug_wabt_validate_ffi(data: &[u8]) -> bool {
    wabt_ffi::fuzz_wabt_validate_ffi(data)
}

// TODO(RM4) - mod cranelift

// TODO(RM3) - https://docs.rs/wast/14.0.0/wast/index.html
