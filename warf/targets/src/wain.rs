/***********************************************
wain
- https://github.com/rhysd/wain
************************************************/

use wain_syntax_binary::parse;
use wain_validate::validate;

pub fn fuzz_wain_parser(data: &[u8]) -> bool {
    // Parse binary into syntax tree
    match parse(&data) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn fuzz_wain_validate(data: &[u8]) -> bool {
    // Parse binary into syntax tree
    match parse(&data) {
        // Validate module
        Ok(tree) => validate(&tree).is_ok(),
        Err(_) => false,
    }
}
