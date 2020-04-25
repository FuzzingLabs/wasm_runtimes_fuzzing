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
