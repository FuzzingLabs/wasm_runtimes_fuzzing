/***********************************************
BINARYEN (Rust binding using FFI)
- https://github.com/pepyakin/binaryen-rs
************************************************/

/// Fuzzing `binaryen::Module` read
///
/// NOTE: We are fuzzing binaryen over FFI.
/// TODO: Verify that binary contains coverage for C++ code of binaryen.
pub fn fuzz_binaryen_ffi(data: &[u8]) -> bool {
    use binaryen::Module;

    Module::read(&data).is_ok()
}

pub fn fuzz_binaryen_optimize_ffi(data: &[u8]) -> bool {
    use binaryen::{CodegenConfig, Module};

    let mut module = match Module::read(&data) {
        Ok(o) => o,
        Err(_) => return false,
    };
    let config = CodegenConfig {
        optimization_level: 4,
        shrink_level: 0,
        debug_info: true,
    };
    module.optimize(&config);
    true
}
