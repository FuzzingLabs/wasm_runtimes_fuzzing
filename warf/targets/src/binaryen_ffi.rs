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

// TODO - binaryen::Module::optimize i.e. wasm-opt
