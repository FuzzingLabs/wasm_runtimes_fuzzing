/***********************************************
WASMPARSER:
- https://github.com/bytecodealliance/wasmparser
************************************************/

/// `Fuzzing wasmparser::Parser` and loop to read all module.
pub fn fuzz_wasmparser_parser(data: &[u8]) -> bool {
    use wasmparser::Parser;
    let res_iter = Parser::new(0).parse_all(&data);
    for res in res_iter {
        match res {
            Err(_) => return false,
            _ => (),
        }
    }
    true
}

/// Fuzzing `wasmparser::ValidatingParser` and loop to read all module.
pub fn fuzz_wasmparser_validate(data: &[u8]) -> bool {
    use wasmparser::validate;

    validate(&data).is_ok()
}

/// Fuzzing `wasmparser::ValidatingParser` with all features enabled and loop to read all module.
pub fn fuzz_wasmparser_validate_all_feat(data: &[u8]) -> bool {
    use wasmparser::Validator;

    let mut validator = Validator::new();
    validator.wasm_reference_types(true);
    validator.wasm_multi_value(true);
    validator.wasm_threads(true);
    validator.wasm_simd(true);
    validator.wasm_module_linking(true);
    validator.wasm_tail_call(true);
    validator.wasm_bulk_memory(true);
    validator.deterministic_only(true);

    // validate
    validator.validate_all(&data).is_ok()
}
