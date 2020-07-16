/***********************************************
wat
- https://github.com/bytecodealliance/wasm-tools/tree/master/crates/wat
************************************************/

pub fn wat_parser(data: &[u8]) -> bool {
    let data = match std::str::from_utf8(&data) {
        Ok(o) => o,
        Err(_) => return false,
    };
    wat::parse_str(&data).is_ok()
}
