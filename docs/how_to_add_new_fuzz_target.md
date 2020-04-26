# How to integrate a new target/project?

In this example, we will learn how to add integrate `wasmer` to WARF and fuzz `wasmer_runtime::validate` method.

## 1. Add the target to `Cargo.toml`

First, we need to:
- Open the file `warf/targets/Cargo.toml`.
- Add `wasmer-runtime = "0.16.2"` to `dependencies` section.

Example:
``` rust
[dependencies]
wasmer-runtime = "0.16.2"
```

## 2. Create the fuzzing function.

Secondly, we need to create our fuzzing function:
- Create a new file inside `warf/targets/src/` (e.g `wasmer.rs`)
- Create a new public function.
- You can add `extern crate` inside the function but it's not always mandatory.
- Call the targetted function and provide `data` to it.

Example:
``` rust
pub fn wasmer_validate(data: &[u8]) {
	extern crate wasmer_runtime;
    let _ = wasmer_runtime::validate(&data);
}
```

Finally, make this function pujblic for fuzzers:
- Open the file `warf/targets/src/lib.rs`.
- add `mod` followed by the name of your previous file.
- add a public function starting with the name `fuzz_` followed by target name.
- Inside this function, call the function you want to fuzz inside `wasmer.rs`

Example:
``` rust
mod wasmer;
pub fn fuzz_wasmer_validate(data: &[u8]) {
    wasmer::wasmer_validate(data);
}
```

## 3. Verify your target is available

Additionnaly, you can verify this new target is listed when using warf `list-targets` subcommand. 

``` sh
$ cargo run list-targets
parity_wasm_deserialize
wasmer_validate
```

## 4. Test your target (with warf debug subcommand)

Verify that your target is working properly using the warf `debug` subcommand. 

``` sh
$ ./warf debug wasmer_validate
[...]
Finished dev [unoptimized + debuginfo] target(s) in 2.47s
$ ./target/debug/debug_wasmer_validate ../corpora/fib.wasm
Start wasmer_validate debug
file_path: "../corpora/fib.wasm"
Everything is OK
```

## 5. Start fuzzing

``` sh
# default fuzzing engine is honggfuzz
$ ./warf target wasmer_validate
```