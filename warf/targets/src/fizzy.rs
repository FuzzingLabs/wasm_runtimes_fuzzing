/***********************************************
fizzy
- https://github.com/wasmx/fizzy
************************************************/

use fizzy::validate;

pub fn fizzy_validate(data: &[u8]) -> bool {
    // Parse binary into syntax tree
    validate(&data)
}
