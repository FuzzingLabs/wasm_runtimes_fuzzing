/***********************************************
WABT (Rust binding using FFI)
- https://github.com/pepyakin/wabt-rs
************************************************/

/// Fuzzing `wabt::wasm2wat_with_features` with all features enabled.
///
/// NOTE: We are fuzzing binaryen over FFI.
/// TODO: Verify if this implementation validate module first.
/// TODO: Verify that binary contains coverage for C++ code of wabt.
pub fn fuzz_wabt_wasm2wat_all_feat_ffi(data: &[u8]) -> bool {
    use wabt::{wasm2wat_with_features, Features};

    let mut features = Features::new();
    features.enable_all();
    wasm2wat_with_features(&data, features).is_ok()
}

/// Fuzzing `wabt::Module::{read_binary, validate}` with default features.
pub fn fuzz_wabt_validate_ffi(data: &[u8]) -> bool {
    use wabt::{Module, ReadBinaryOptions};

    // Default wasm features sets by `Module::read_binary`.
    match Module::read_binary(&data, &ReadBinaryOptions::default()) {
        Ok(module) => module.validate().is_ok(),
        _ => false,
    }
}

pub fn fuzz_wabt_wat2wasm_ffi(data: &[u8]) -> bool {
    use wabt::{wat2wasm_with_features, Features};

    let mut features = Features::new();
    features.enable_all();

    wat2wasm_with_features(&data, features).is_ok()
}

// TODO(RM4) - Module::parse_wat
