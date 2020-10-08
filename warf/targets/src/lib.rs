/*
Differential fuzzing:
We are checking that all those different implementation return
the same thing i.e. true or false in our cases.
*/

pub fn fuzz_diff_parsing(data: &[u8]) {
    let a = parity_wasm::parity_wasm_deserialize(&data);
    let b = wasmer::fuzz_wasmer_compile_clif(&data);
    let c = wasmer::fuzz_wasmer_compile_singlepass(&data);
    let d = wasmtime::fuzz_wasmtime_compile_all_cranelift(&data);
    let e = wasmparser::fuzz_wasmparser_validate_all_feat(&data);
    let f = binaryen_ffi::fuzz_binaryen_ffi(&data);
    let g = wasmprinter::fuzz_wasmprinter_parser(&data);

    let _ = match (a, b, c, d, e, f, g) {
        (true, true, true, true, true, true, true) => true,
        (false, false, false, false, false, false, false) => false,
        _ => panic!(
            "fuzz_diff_parsing panic: {}-{}-{}-{}-{}-{}-{}",
            a, b, c, d, e, f, g
        ),
    };
}

pub fn debug_diff_parsing(data: &[u8]) -> bool {
    let a = parity_wasm::parity_wasm_deserialize(&data);
    let b = wasmer::fuzz_wasmer_compile_clif(&data);
    let c = wasmer::fuzz_wasmer_compile_singlepass(&data);
    let d = wasmtime::fuzz_wasmtime_compile_all_cranelift(&data);
    let e = wasmparser::fuzz_wasmparser_validate_all_feat(&data);
    let f = binaryen_ffi::fuzz_binaryen_ffi(&data);
    let g = wasmprinter::fuzz_wasmprinter_parser(&data);

    match (a, b, c, d, e, f, g) {
        (true, true, true, true, true, true, true) => true,
        (false, false, false, false, false, false, false) => true,
        _ => false,
    }
}

pub fn fuzz_diff_all_validate(data: &[u8]) {
    let a = wasmi::wasmi_validate(&data);
    let b = wasmer::fuzz_wasmer_validate(&data);
    let c = wasmtime::fuzz_wasmtime_validate_all_feat(&data);
    let d = wasmparser::fuzz_wasmparser_validate_all_feat(&data);
    let e = wabt_ffi::fuzz_wabt_validate_ffi(&data);
    let f = fizzy::fizzy_validate(&data);

    let _ = match (a, b, c, d, e, f) {
        (true, true, true, true, true, true) => true,
        (false, false, false, false, false, false) => false,
        _ => panic!(
            "fuzz_diff_all_validate panic: {}-{}-{}-{}-{}-{}",
            a, b, c, d, e, f
        ),
    };
}

pub fn debug_diff_all_validate(data: &[u8]) -> bool {
    let a = wasmi::wasmi_validate(&data);
    let b = wasmer::fuzz_wasmer_validate(&data);
    let c = wasmtime::fuzz_wasmtime_validate_all_feat(&data);
    let d = wasmparser::fuzz_wasmparser_validate_all_feat(&data);
    let e = wabt_ffi::fuzz_wabt_validate_ffi(&data);
    let f = fizzy::fizzy_validate(&data);

    match (a, b, c, d, e, f) {
        (true, true, true, true, true, true) => true,
        (false, false, false, false, false, false) => true,
        _ => false,
    }
}

pub fn fuzz_diff_instantiate(data: &[u8]) {
    let a = wasmi::wasmi_instantiate(&data);
    let b = wasmer::fuzz_wasmer_instantiate(&data);
    let c = wasmtime::fuzz_wasmtime_instantiate_all_cranelift(&data);
    let _ = match (a, b, c) {
        (true, true, true) => true,
        (false, false, false) => false,
        _ => panic!("fuzz_diff_instantiate panic: {}-{}-{}", a, b, c),
    };
}

pub fn debug_diff_instantiate(data: &[u8]) -> bool {
    let a = wasmi::wasmi_instantiate(&data);
    let b = wasmer::fuzz_wasmer_instantiate(&data);
    let c = wasmtime::fuzz_wasmtime_instantiate_all_cranelift(&data);
    match (a, b, c) {
        (true, true, true) => true,
        (false, false, false) => true,
        _ => false,
    }
}

pub fn fuzz_diff_wat_parsing(data: &[u8]) {
    let a = wabt_ffi::fuzz_wabt_wat2wasm_ffi(&data);
    let b = wat::wat_parser(&data);
    let _ = match (a, b) {
        (true, true) => true,
        (false, false) => false,
        _ => panic!("fuzz_diff_wat_parsing panic: {}-{}", a, b),
    };
}

pub fn debug_diff_wat_parsing(data: &[u8]) -> bool {
    let a = wabt_ffi::fuzz_wabt_wat2wasm_ffi(&data);
    let b = wat::wat_parser(&data);
    match (a, b) {
        (true, true) => true,
        (false, false) => true,
        _ => false,
    }
}

mod wasmi;
// fuzzing harnesses
pub fn fuzz_wasmi_validate(data: &[u8]) {
    let _ = wasmi::wasmi_validate(&data);
}
pub fn fuzz_wasmi_instantiate(data: &[u8]) {
    let _ = wasmi::wasmi_instantiate(&data);
}
// debug target
pub fn debug_wasmi_validate(data: &[u8]) -> bool {
    wasmi::wasmi_validate(&data)
}
pub fn debug_wasmi_instantiate(data: &[u8]) -> bool {
    wasmi::wasmi_instantiate(&data)
}

mod parity_wasm;
// fuzzing harnesses
pub fn fuzz_parity_wasm_deserialize(data: &[u8]) {
    let _ = parity_wasm::parity_wasm_deserialize(&data);
}
// debug target
pub fn debug_parity_wasm_deserialize(data: &[u8]) -> bool {
    parity_wasm::parity_wasm_deserialize(&data)
}

mod wasmer;
// fuzzing harnesses
pub fn fuzz_wasmer_validate(data: &[u8]) {
    let _ = wasmer::fuzz_wasmer_validate(&data);
}
pub fn fuzz_wasmer_compile_clif(data: &[u8]) {
    let _ = wasmer::fuzz_wasmer_compile_clif(&data);
}
pub fn fuzz_wasmer_compile_singlepass(data: &[u8]) {
    let _ = wasmer::fuzz_wasmer_compile_singlepass(&data);
}
pub fn fuzz_wasmer_instantiate(data: &[u8]) {
    let _ = wasmer::fuzz_wasmer_instantiate(&data);
}
// debug target
pub fn debug_wasmer_validate(data: &[u8]) -> bool {
    wasmer::fuzz_wasmer_validate(&data)
}
pub fn debug_wasmer_compile_clif(data: &[u8]) -> bool {
    wasmer::fuzz_wasmer_compile_clif(&data)
}
pub fn debug_wasmer_compile_singlepass(data: &[u8]) -> bool {
    wasmer::fuzz_wasmer_compile_singlepass(&data)
}
pub fn debug_wasmer_instantiate(data: &[u8]) -> bool {
    wasmer::fuzz_wasmer_instantiate(&data)
}

mod wasmtime;
// fuzzing harnesses
pub fn fuzz_wasmtime_validate(data: &[u8]) {
    let _ = wasmtime::fuzz_wasmtime_validate(&data);
}
pub fn fuzz_wasmtime_validate_all_feat(data: &[u8]) {
    let _ = wasmtime::fuzz_wasmtime_validate_all_feat(&data);
}
pub fn fuzz_wasmtime_compile(data: &[u8]) {
    let _ = wasmtime::fuzz_wasmtime_compile(&data);
}
pub fn fuzz_wasmtime_compile_all_cranelift(data: &[u8]) {
    let _ = wasmtime::fuzz_wasmtime_compile_all_cranelift(&data);
}
/* DEACTIVATED FOR NOW
pub fn _fuzz_wasmtime_compile_all_lightbeam(data: &[u8]) {
    let _ = wasmtime::fuzz_wasmtime_compile_all_lightbeam(data);
}
*/
pub fn fuzz_wasmtime_instantiate_all_cranelift(data: &[u8]) {
    let _ = wasmtime::fuzz_wasmtime_instantiate_all_cranelift(&data);
}
/* DEACTIVATED FOR NOW
pub fn _fuzz_wasmtime_instantiate_all_lightbeam(data: &[u8]) {
    let _ = wasmtime::fuzz_wasmtime_instantiate_all_lightbeam(data);
}
*/
// debug target
pub fn debug_wasmtime_validate(data: &[u8]) -> bool {
    wasmtime::fuzz_wasmtime_validate(&data)
}
pub fn debug_wasmtime_validate_all_feat(data: &[u8]) -> bool {
    wasmtime::fuzz_wasmtime_validate_all_feat(&data)
}
pub fn debug_wasmtime_compile(data: &[u8]) -> bool {
    wasmtime::fuzz_wasmtime_compile(&data)
}
pub fn debug_wasmtime_compile_all_cranelift(data: &[u8]) -> bool {
    wasmtime::fuzz_wasmtime_compile_all_cranelift(&data)
}
/* DEACTIVATED FOR NOW
pub fn _debug_wasmtime_compile_all_lightbeam(data: &[u8]) -> bool {
    wasmtime::fuzz_wasmtime_compile_all_lightbeam(data)
}
*/
pub fn debug_wasmtime_instantiate_all_cranelift(data: &[u8]) -> bool {
    wasmtime::fuzz_wasmtime_instantiate_all_cranelift(&data)
}
/* DEACTIVATED FOR NOW
pub fn _debug_wasmtime_instantiate_all_lightbeam(data: &[u8]) -> bool {
    wasmtime::fuzz_wasmtime_instantiate_all_lightbeam(data)
}

mod lightbeam;
// fuzzing harnesses
pub fn _fuzz_lightbeam_translate(data: &[u8]) {
    let _ = lightbeam::fuzz_lightbeam_translate(data);
}
// debug target
pub fn _debug_lightbeam_translate(data: &[u8]) -> bool {
    lightbeam::fuzz_lightbeam_translate(data)
}
*/

mod wasmparser;
// fuzzing harnesses
pub fn fuzz_wasmparser_parser(data: &[u8]) {
    let _ = wasmparser::fuzz_wasmparser_parser(&data);
}
pub fn fuzz_wasmparser_validate(data: &[u8]) {
    let _ = wasmparser::fuzz_wasmparser_validate(&data);
}
pub fn fuzz_wasmparser_validate_all_feat(data: &[u8]) {
    let _ = wasmparser::fuzz_wasmparser_validate_all_feat(&data);
}
// debug target
pub fn debug_wasmparser_parser(data: &[u8]) -> bool {
    wasmparser::fuzz_wasmparser_parser(&data)
}
pub fn debug_wasmparser_validate(data: &[u8]) -> bool {
    wasmparser::fuzz_wasmparser_validate(&data)
}
pub fn debug_wasmparser_validate_all_feat(data: &[u8]) -> bool {
    wasmparser::fuzz_wasmparser_validate_all_feat(&data)
}

mod binaryen_ffi;
// fuzzing harnesses
pub fn fuzz_binaryen_ffi(data: &[u8]) {
    let _ = binaryen_ffi::fuzz_binaryen_ffi(&data);
}
pub fn fuzz_binaryen_optimize_ffi(data: &[u8]) {
    let _ = binaryen_ffi::fuzz_binaryen_optimize_ffi(&data);
}
// debug target
pub fn debug_binaryen_ffi(data: &[u8]) -> bool {
    binaryen_ffi::fuzz_binaryen_ffi(&data)
}
pub fn debug_binaryen_optimize_ffi(data: &[u8]) -> bool {
    binaryen_ffi::fuzz_binaryen_optimize_ffi(&data)
}

mod wabt_ffi;

pub fn fuzz_wabt_wasm2wat_all_feat_ffi(data: &[u8]) {
    let _ = wabt_ffi::fuzz_wabt_wasm2wat_all_feat_ffi(&data);
}
pub fn fuzz_wabt_validate_ffi(data: &[u8]) {
    let _ = wabt_ffi::fuzz_wabt_validate_ffi(&data);
}
pub fn fuzz_wabt_wat2wasm_ffi(data: &[u8]) {
    let _ = wabt_ffi::fuzz_wabt_wat2wasm_ffi(&data);
}

// debug target
pub fn debug_wabt_wasm2wat_all_feat_ffi(data: &[u8]) -> bool {
    wabt_ffi::fuzz_wabt_wasm2wat_all_feat_ffi(&data)
}
pub fn debug_wabt_validate_ffi(data: &[u8]) -> bool {
    wabt_ffi::fuzz_wabt_validate_ffi(&data)
}
pub fn debug_wabt_wat2wasm_ffi(data: &[u8]) -> bool {
    wabt_ffi::fuzz_wabt_wat2wasm_ffi(&data)
}

mod wasm3;
pub fn fuzz_wasm3_parser_ffi(data: &[u8]) {
    let _ = wasm3::fuzz_wasm3_parser_ffi(&data);
}
pub fn debug_wasm3_parser_ffi(data: &[u8]) -> bool {
    wasm3::fuzz_wasm3_parser_ffi(&data)
}

mod wasmprinter;
pub fn fuzz_wasmprinter_parser(data: &[u8]) {
    let _ = wasmprinter::fuzz_wasmprinter_parser(&data);
}

pub fn debug_wasmprinter_parser(data: &[u8]) -> bool {
    wasmprinter::fuzz_wasmprinter_parser(&data)
}

mod wain;
pub fn fuzz_wain_parser(data: &[u8]) {
    let _ = wain::fuzz_wain_parser(&data);
}
pub fn fuzz_wain_validate(data: &[u8]) {
    let _ = wain::fuzz_wain_validate(&data);
}

pub fn debug_wain_parser(data: &[u8]) -> bool {
    wain::fuzz_wain_parser(&data)
}
pub fn debug_wain_validate(data: &[u8]) -> bool {
    wain::fuzz_wain_validate(&data)
}

mod wat;
pub fn fuzz_wat_parser(data: &[u8]) {
    let _ = wat::wat_parser(&data);
}
pub fn debug_wat_parser(data: &[u8]) -> bool {
    wat::wat_parser(&data)
}

mod wast;
pub fn fuzz_wast_parser(data: &[u8]) {
    let _ = wast::wast_parser(&data);
}
pub fn debug_wast_parser(data: &[u8]) -> bool {
    wast::wast_parser(&data)
}

mod fizzy;
pub fn fuzz_fizzy_validate(data: &[u8]) {
    let _ = fizzy::fizzy_validate(&data);
}
pub fn debug_fizzy_validate(data: &[u8]) -> bool {
    fizzy::fizzy_validate(&data)
}
