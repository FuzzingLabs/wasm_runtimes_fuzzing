/***********************************************
WASMTIME:
- https://github.com/bytecodealliance/wasmtime
***********************************************/

/// Fuzzing `wasmtime::validate` with default Store/Config/Engine
pub fn fuzz_wasmtime_validate(data: &[u8]) {
    use wasmtime::{Module, Store};
    let store = Store::default();
    let _module = Module::validate(&store, &data);
}

/// Fuzzing `wasmtime::Module` with default Store/Config/Engine
///
/// NOTE: wasmtime::from_binary also called wasmtime::validate.
pub fn fuzz_wasmtime_compile(data: &[u8]) {
    use wasmtime::{Module, Store};
    let store = Store::default();
    let _module = Module::from_binary(&store, &data);
}

use wasmtime::Strategy;
fn fuzz_wasmtime_all(data: &[u8], strategy: Strategy) {
    use wasmtime::{Config, Engine, Module, Store};
    // Create new compilation config
    let mut config = Config::new();
    // Select Cranelift as compiler
    match config.strategy(strategy) {
        Ok(o) => o,
        _ => return,
    };
    // Activate all wasm features in Config
    // https://docs.rs/wasmtime/0.15.0/wasmtime/struct.Config.html
    config
        .debug_info(true)
        .wasm_threads(true)
        .wasm_reference_types(true)
        .wasm_simd(true)
        .wasm_bulk_memory(true)
        .wasm_multi_value(true);
    let store = Store::new(&Engine::new(&config));
    // Will first validate then creates a new Module
    let _module = Module::from_binary(&store, &data);
}

/// Fuzzing `wasmtime::Module` with all wasm features and `Cranelift` backend.
pub fn fuzz_wasmtime_all_cranelift(data: &[u8]) {
    fuzz_wasmtime_all(data, Strategy::Cranelift);
}

/// Fuzzing `wasmtime::Module` with all wasm features and `Lightbeam` backend.
pub fn fuzz_wasmtime_all_lightbeam(data: &[u8]) {
    fuzz_wasmtime_all(data, Strategy::Lightbeam);
}

// TODO - create new fuzz targets with other config values
// TODO - instantiate for Cranelift and Lightbeam.
