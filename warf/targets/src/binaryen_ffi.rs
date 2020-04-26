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

pub fn fuzz_binaryen_optimize_ffi(data: &[u8]) {
    use binaryen::{CodegenConfig, Module};
    let config = CodegenConfig {
        optimization_level: 4,
        shrink_level: 0,
        debug_info: true,
    };
    let mut module = match Module::read(&data) {
        Ok(o) => o,
        Err(_) => return,
    };
    module.optimize(&config);
}
