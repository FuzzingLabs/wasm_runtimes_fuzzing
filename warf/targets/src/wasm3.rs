/***********************************************
wasm3 (Rust binding using FFI)
- https://github.com/Veykril/wasm3-rs
************************************************/

use wasm3::Environment;
use wasm3::Module;

pub fn fuzz_wasm3_parser_ffi(data: &[u8]) -> bool {
    let env = Environment::new().expect("Unable to create environment");
    let _rt = env
        .create_runtime(1024 * 60)
        .expect("Unable to create runtime");
    Module::parse(&env, &data).is_ok()
}
