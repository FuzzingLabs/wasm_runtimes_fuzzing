# How to integrate a new target/project?

In this example, we will learn how to add integrate `wasmer` to WARF and fuzz `wasmer_runtime::validate` method.

## 1. Add the target to `Cargo.toml`

First, we need to:
- Open the file `warf/common/Cargo.toml`.
- Add `wasmer-runtime = "0.16.2"` to `dependencies` section.

Example:
``` rust
[dependencies]
wasmer-runtime = "0.16.2"
```

## 2. Create the fuzzing function.

Secondly, we need to create our fuzzing function:
- Open the file `warf/common/src/lib.rs`.
- Create a new function starting with the name `fuzz_` followed by target name.
- You can add `extern crate` inside the function but it's not always mandatory.
- Call the targetted function and provide `data` to it.

Example:
``` rust
pub fn fuzz_wasmer_validate(data: &[u8]) {
	extern crate wasmer_runtime;
    let _ = wasmer_runtime::validate(&data);
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
$ cargo run debug wasmer_validate
[...]
Finished dev [unoptimized + debuginfo] target(s) in 2.47s
$ ./target/debug/debug_wasmer_validate ../corpora/fib.wasm
Start wasmer_validate debug
file_path: "../corpora/fib.wasm"
Everything is OK
```

## 5. Start fuzzing

``` sh
$ cargo run target wasmer_validate
```