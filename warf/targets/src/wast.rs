/***********************************************
wast
- https://github.com/bytecodealliance/wasm-tools/tree/master/crates/wast
************************************************/

pub fn wast_parser(data: &[u8]) -> bool {
    use wast::parser::{self, ParseBuffer};
    use wast::Wat;

    let data = match std::str::from_utf8(&data) {
        Ok(o) => o,
        Err(_) => return false,
    };

    let buf = match ParseBuffer::new(&data) {
        Ok(o) => o,
        Err(_) => return false,
    };
    parser::parse::<Wat>(&buf).is_ok()
}
