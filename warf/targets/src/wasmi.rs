/***********************************************
wasmi:
- https://github.com/paritytech/wasmi
************************************************/

/// Fuzzing `wasmi::validate_module`.
pub fn wasmi_validate(data: &[u8]) -> bool {
    use parity_wasm::{deserialize_buffer, elements};
    use wasmi_validation::{validate_module, PlainValidator};

    let module: elements::Module = match deserialize_buffer(&data) {
        Ok(module) => module,
        _ => return false,
    };
    validate_module::<PlainValidator>(&module).is_ok()
}

/// Fuzzing `wasmi::ModuleInstance` with default `ImportsBuilder`.
pub fn wasmi_instantiate(data: &[u8]) -> bool {
    use wasmi::{ImportsBuilder, Module, ModuleInstance};

    match Module::from_buffer(&data) {
        Ok(module) => ModuleInstance::new(&module, &ImportsBuilder::default()).is_ok(),
        _ => false,
    }

    // TODO(RM3): add calls to instance functions like:
    // - invoke_export: https://github.com/paritytech/wasmi/blob/b67af25899874de7aac187e08e3b2a30d9bbc388/benches/src/lib.rs#L38
    // - run_start: https://github.com/paritytech/wasmi/blob/899cc32e45483fce12907f807ee9b09d837d2636/examples/interpret.rs#L36
}
