/***********************************************
parity-wasm:
- https://github.com/paritytech/parity-wasm
************************************************/

/// Fuzzing `parity_wasm::deserialize_buffer`.
///
/// TODO: Will be more useful with calls to `parity_wasm::elements::Module` accessors.
/// NOTE: wasmi already use this function in `wasmi::Module::from_buffer`.
pub fn fuzz_parity_wasm_deserialize(data: &[u8]) {
    use parity_wasm::{deserialize_buffer, elements};

    let _module: std::result::Result<elements::Module, _> = deserialize_buffer(&data);
}

// TODO(RM4) - parity_wasm::elements::serialize
