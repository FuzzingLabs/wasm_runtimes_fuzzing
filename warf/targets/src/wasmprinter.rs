/***********************************************
wasmprinter
- https://github.com/bytecodealliance/wasm-tools/tree/master/crates/wasmprinter
************************************************/

pub fn fuzz_wasmprinter_parser(data: &[u8]) -> bool {
    wasmprinter::print_bytes(&data).is_ok()
}
