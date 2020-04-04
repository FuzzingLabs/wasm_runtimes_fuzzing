#[macro_use] extern crate afl;
extern crate fuzz_targets_common;
use fuzz_targets_common::fuzz_###TARGET### as fuzz_target;

fn main() {
    fuzz!(|data|{
        fuzz_target(&data);
    });
}
