/***********************************************
parity-wasm:
- https://github.com/paritytech/parity-wasm
************************************************/

/// Fuzzing `parity_wasm::deserialize_buffer`.
///
/// TODO: Will be more useful with calls to `parity_wasm::elements::Module` accessors.
/// NOTE: wasmi already use this function in `wasmi::Module::from_buffer`.
pub fn parity_wasm_deserialize(data: &[u8]) -> bool {
    use parity_wasm::{deserialize_buffer, elements};

    let module: std::result::Result<elements::Module, _> = deserialize_buffer(&data);
    module.is_ok()
}
