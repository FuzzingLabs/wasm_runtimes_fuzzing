/***********************************************
WASMTIME:
- https://github.com/bytecodealliance/wasmtime
***********************************************/

use wasmtime::{Config, Engine, Instance, Module, Store, Strategy};

/// Fuzzing `wasmtime::validate` with default Store/Config/Engine
pub fn fuzz_wasmtime_validate(data: &[u8]) {
    let store = Store::default();
    let _module = Module::validate(&store, &data);
}

/// Fuzzing `wasmtime::validate` with all the features enabled
pub fn fuzz_wasmtime_validate_all_feat(data: &[u8]) {
    let store = match get_store_all_feat(Strategy::Cranelift) {
        None => return,
        Some(a) => a,
    };
    let _module = Module::validate(&store, &data);
}

/// Fuzzing `wasmtime::Module` with default Store/Config/Engine
///
/// NOTE: wasmtime::from_binary is also calling wasmtime::validate.
pub fn fuzz_wasmtime_compile(data: &[u8]) {
    let store = Store::default();
    let _module = Module::from_binary(&store, &data);
}

/// Return a Store created with the given Strategy and with
/// all the features enabled
fn get_store_all_feat(strategy: Strategy) -> Option<Store> {
    // Create new compilation config
    let mut config = Config::new();
    // Select Cranelift as compiler
    match config.strategy(strategy) {
        Ok(o) => o,
        _ => return None,
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
    Some(store)
}

/// Fuzzing `wasmtime::Module` with all wasm features and `Cranelift` backend.
pub fn fuzz_wasmtime_compile_all_cranelift(data: &[u8]) {
    let store = match get_store_all_feat(Strategy::Cranelift) {
        None => return,
        Some(a) => a,
    };
    let _module = Module::from_binary(&store, &data);
}

/// Fuzzing `wasmtime::Module` with all wasm features and `Lightbeam` backend.
pub fn fuzz_wasmtime_compile_all_lightbeam(data: &[u8]) {
    let store = match get_store_all_feat(Strategy::Lightbeam) {
        None => return,
        Some(a) => a,
    };
    let _module = Module::from_binary(&store, &data);
}

/// Fuzzing `wasmtime::Instance` with all wasm features and `Cranelift` backend.
pub fn fuzz_wasmtime_instantiate_all_cranelift(data: &[u8]) {
    let store = match get_store_all_feat(Strategy::Cranelift) {
        None => return,
        Some(a) => a,
    };
    // Create a Module
    let module = match Module::from_binary(&store, &data) {
        Ok(a) => a,
        _ => return,
    };
    let _instance = Instance::new(&module, &[]);
    // TODO(RM4) - check parameter Instance
    // TODO(RM4) - Execute function of the module
}

/// Fuzzing `wasmtime::Instance` with all wasm features and `Lightbeam` backend.
pub fn fuzz_wasmtime_instantiate_all_lightbeam(data: &[u8]) {
    let store = match get_store_all_feat(Strategy::Lightbeam) {
        None => return,
        Some(a) => a,
    };
    // Create a Module
    let module = match Module::from_binary(&store, &data) {
        Ok(a) => a,
        _ => return,
    };
    let _instance = Instance::new(&module, &[]);
    // TODO(RM4) - check parameter Instance
    // TODO(RM4) - Execute function of the module
}
