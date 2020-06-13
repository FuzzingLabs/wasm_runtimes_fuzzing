/***********************************************
wain
- https://github.com/rhysd/wain
************************************************/

use wain_syntax_binary::parse;
use wain_validate::validate;

pub fn fuzz_wain_parser(data: &[u8]) -> bool {
    // Parse binary into syntax tree
    let _tree = match parse(&data) {
        Ok(_tree) => return true,
        Err(_err) => return false,
    };
}

pub fn fuzz_wain_validate(data: &[u8]) -> bool {
    // Parse binary into syntax tree
    let tree = match parse(&data) {
        Ok(tree) => tree,
        Err(_err) => {
            return false;
        }
    };

    // Validate module
    validate(&tree).is_ok()
}
