/***********************************************
WASMPARSER:
- https://github.com/bytecodealliance/wasmparser
************************************************/

/// `Fuzzing wasmparser::Parser` and loop to read all module.
pub fn fuzz_wasmparser_parser(data: &[u8]) {
    use wasmparser::{Parser, ParserState, WasmDecoder};
    let mut parser = Parser::new(data);
    loop {
        match *parser.read() {
            ParserState::Error(..) | ParserState::EndWasm => break,
            _ => (),
        }
    }
}

/// Fuzzing `wasmparser::ValidatingParser` and loop to read all module.
pub fn fuzz_wasmparser_validate(data: &[u8]) {
    use wasmparser::{ParserState, ValidatingParser, WasmDecoder};
    let mut parser = ValidatingParser::new(data, None);
    loop {
        match *parser.read() {
            ParserState::Error(..) | ParserState::EndWasm => break,
            _ => (),
        }
    }
}

// TODO - wasmparser::ValidatingParserConfig
