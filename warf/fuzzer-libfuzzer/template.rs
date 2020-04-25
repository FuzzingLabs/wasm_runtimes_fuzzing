#![no_main]

#[macro_use]
extern crate libfuzzer_sys;
extern crate fuzz_targets;
use fuzz_targets::fuzz_###TARGET### as fuzz_target;

fuzz_target!(|data|{
    fuzz_target(data);
});
