/***********************************************
WASMPARSER:
- https://github.com/bytecodealliance/wasmparser
************************************************/

/// `Fuzzing wasmparser::Parser` and loop to read all module.
pub fn fuzz_wasmparser_parser(data: &[u8]) -> bool {
    use wasmparser::{Parser, ParserState, WasmDecoder};
    let mut parser = Parser::new(data);
    loop {
        match *parser.read() {
            ParserState::Error(..) => return false,
            ParserState::EndWasm => break,
            _ => (),
        }
    }
    true
}

/// Fuzzing `wasmparser::ValidatingParser` and loop to read all module.
pub fn fuzz_wasmparser_validate(data: &[u8]) -> bool {
    use wasmparser::{ParserState, ValidatingParser, WasmDecoder};
    let mut parser = ValidatingParser::new(data, None);
    loop {
        match *parser.read() {
            ParserState::Error(..) => return false,
            ParserState::EndWasm => break,
            _ => (),
        }
    }
    true
}

/// Fuzzing `wasmparser::ValidatingParser` with all features enabled and loop to read all module.
pub fn fuzz_wasmparser_validate_all_feat(data: &[u8]) -> bool {
    use wasmparser::{
        OperatorValidatorConfig, ParserState, ValidatingParser, ValidatingParserConfig, WasmDecoder,
    };

    // activate all features
    let validator_config: Option<ValidatingParserConfig> = Some(ValidatingParserConfig {
        operator_config: OperatorValidatorConfig {
            enable_threads: true,
            enable_reference_types: true,
            enable_simd: true,
            enable_bulk_memory: true,
            enable_multi_value: true,
            enable_tail_call: true,
        },
    });

    let mut parser = ValidatingParser::new(data, validator_config);
    loop {
        match *parser.read() {
            ParserState::Error(..) => return false,
            ParserState::EndWasm => break,
            _ => (),
        }
    }
    true
}
