# WARF SUBCOMMANDS

The main goal of this tool is to provide an easy way to fuzz WebAssembly VMs/runtimes.
In addition to finding bugs, fuzzing will also generated samples/inputs that can be reused as unittest.


Main features are:
- Automatic fuzzing of runtimes harnesses (without any user interaction)
- Multiple fuzzing engines available (honggfuzz, afl, libfuzzer)
- Multi-threading (depending of the fuzzer, only honggfuzz for now)
- Crash report/detection


# Subcommands

List all subcommands with `help`:
``` sh
$ ./warf help

[...]
SUBCOMMANDS:
    benchmark-all    Run WebAssembly module on all targets with benchmark
    build            Build all targets for this specific fuzzer
    continuously     Run all fuzz targets
    debug            Debug one target
    execute-all      Run WebAssembly module on all targets
    help             Prints this message or the help of the given subcommand(s)
    list             List all available targets
    target           Run one target with specific fuzzer
```

## List available targets (`list`)

Available fuzzing targets can be listed with:
```sh
$ ./warf list

wasmi_validate
wasmi_instantiate
parity_wasm_deserialize
[...]
binaryen_ffi
wabt_wasm2wat_all_feat_ffi
wabt_validate_ffi
```

## Fuzzing one target (`target`)

- Run fuzzing on a target (default fuzzing engine is `honggfuzz`):
``` sh
$ ./warf target wasmer_validate

[...]

------------------------[  0 days 00 hrs 00 mins 02 secs ]----------------------
  Iterations : 272,647 [272.65k]
  Mode [3/3] : Feedback Driven Mode
      Target : hfuzz_target/x86_64-unknown-linux-gnu/release/wasmer_validate
     Threads : 4, CPUs: 8, CPU%: 529% [66%/CPU]
       Speed : 171,238/sec [avg: 136,323]
     Crashes : 0 [unique: 0, blacklist: 0, verified: 0]
    Timeouts : 0 [10 sec]
 Corpus Size : 754, max: 8,192 bytes, init: 1,126 files
  Cov Update : 0 days 00 hrs 00 mins 01 secs ago
    Coverage : edge: 3,194/58,784 [5%] pc: 2 cmp: 41,653
---------------------------------- [ LOGS ] ------------------/ honggfuzz 2.0 /-
Size:77 (i,b,hw,ed,ip,cmp): 0/0/0/1/0/0, Tot:0/0/0/3159/2/41623
[...]
```

- Available options:
``` sh
$ ./warf target wasmer_validate --help

[...]

OPTIONS:
        --fuzzer <fuzzer>      Which fuzzer to run [default: Honggfuzz]  [possible values: Afl, Honggfuzz, Libfuzzer]
    -n, --thread <thread>      Set number of thread (only for hfuzz)
    -t, --timeout <timeout>    Set timeout per target

ARGS:
    <target>    Which target to run
```
NOTE: More informations about advanced options and the different fuzzing engines can be found [here](warf_advanced_options.md).


## Continuous fuzzing (`continuously`)

warf in continuous mode will executed all target once then stop.

Help:
``` sh
$ ./warf continuously --help
Run all fuzz targets

USAGE:
    cli continuously [FLAGS] [OPTIONS]

FLAGS:
        --cargo-update    
    -h, --help            Prints help information
    -i, --infinite        
    -V, --version         Prints version information

OPTIONS:
    -q, --filter <filter>      Only run target containing this string
        --fuzzer <fuzzer>      Which fuzzer to run [default: Honggfuzz]  [possible values: Afl, Honggfuzz, Libfuzzer]
    -n, --thread <thread>      Set number of thread (only for hfuzz)
    -t, --timeout <timeout>    Set timeout per target [default: 10]
```

Prefered command:
``` sh
$ ./warf continuously -i -t 600
# -i => infinite mode.
# -t => timeout of 10 min, will switch to another fuzzing target every 10 min.
```

## Debug one target (`debug`)

This command will create a simple debugging tool for the choosen target allowing you to easily analyze the root cause of a crash as shown inside this tutorial: [how_to_analyze_a_crash.md](how_to_analyze_a_crash.md).

NOTE: all debugging tools generated with `debug` subcommand are available inside: `workspace/debug/target/debug/`.


## Execute one wasm module throw all targets (`execute-all`)

This command will create, build and execute a standalone binary running one wasm module throw all fuzzing targets.
NOTE: This binary will be stored in `workspace/exec_all` allowing you to directly call it later without re-compilation.
NOTE 2: Targets do not handle the same type of data that's why it can be perfectly valid to have a target returning an error e.g. if you provide a wasm module to `wat_parser`, it will return an error.

``` sh
$ ./warf execute-all workspace/corpora/wasm/fib.wasm

[...]

[WARF] execute_all compiled here: "XXX/wasm_runtimes_fuzzing/warf/workspace/exec_all"
Execution of all runtime engine
file_to_process: "workspace/corpora/wasm/fib.wasm"

[O] wasmi_validate: Ok()
[O] wasmi_instantiate: Ok()
[O] parity_wasm_deserialize: Ok()
[O] wasmer_validate: Ok()
[O] wasmer_compile_clif: Ok()
[O] wasmer_compile_singlepass: Ok()
[O] wasmer_instantiate: Ok()
[O] wasmtime_validate: Ok()
[O] wasmtime_validate_all_feat: Ok()
[O] wasmtime_compile: Ok()
[O] wasmtime_compile_all_cranelift: Ok()
[X] wasmtime_compile_all_lightbeam: Err()
[O] wasmtime_instantiate_all_cranelift: Ok()
[X] wasmtime_instantiate_all_lightbeam: Err()
[X] lightbeam_translate: Err()
[O] wasmparser_parser: Ok()
[O] wasmparser_validate: Ok()
[O] wasmparser_validate_all_feat: Ok()
[O] binaryen_ffi: Ok()
[O] binaryen_optimize_ffi: Ok()
[O] wabt_wasm2wat_all_feat_ffi: Ok()
[O] wabt_validate_ffi: Ok()

No crash, everything is OK
```

## Benchmark execution targets speed (`benchmark-all`)

This command will add benchmarking information into `exec_all` binary and produce the following result during execution:

``` sh
$ ./warf benchmark-all workspace/corpora/wasm/fib.wasm

[...]

[WARF] execute_all compiled here: "XXX/wasm_runtimes_fuzzing/warf/workspace/exec_all"
Execution of all runtime engine
file_to_process: "workspace/corpora/wasm/fib.wasm"

[O] wasmi_validate: Ok()
benchmark (sec): 0.0000001458
[O] wasmi_instantiate: Ok()
benchmark (sec): 0.0000000711
[O] parity_wasm_deserialize: Ok()
benchmark (sec): 0.0000000218
[O] wasmer_validate: Ok()
benchmark (sec): 0.0000000573
[O] wasmer_compile_clif: Ok()
benchmark (sec): 0.0000023549
[O] wasmer_compile_singlepass: Ok()
benchmark (sec): 0.0000006215
[O] wasmer_instantiate: Ok()
benchmark (sec): 0.0000009492
[O] wasmtime_validate: Ok()
benchmark (sec): 0.0000002413
[O] wasmtime_validate_all_feat: Ok()
benchmark (sec): 0.0000000390
[O] wasmtime_compile: Ok()
benchmark (sec): 0.0000008680
[O] wasmtime_compile_all_cranelift: Ok()
benchmark (sec): 0.0000009030
[X] wasmtime_compile_all_lightbeam: Err()
benchmark (sec): 0.0000000171
[O] wasmtime_instantiate_all_cranelift: Ok()
benchmark (sec): 0.0000012563
[X] wasmtime_instantiate_all_lightbeam: Err()
benchmark (sec): 0.0000000118
[X] lightbeam_translate: Err()
benchmark (sec): 0.0000000257
[O] wasmparser_parser: Ok()
benchmark (sec): 0.0000000154
[O] wasmparser_validate: Ok()
benchmark (sec): 0.0000000267
[O] wasmparser_validate_all_feat: Ok()
benchmark (sec): 0.0000000147
[O] binaryen_ffi: Ok()
benchmark (sec): 0.0000001865
[O] binaryen_optimize_ffi: Ok()
benchmark (sec): 0.0000063538
[O] wabt_wasm2wat_all_feat_ffi: Ok()
benchmark (sec): 0.0000000897
[O] wabt_validate_ffi: Ok()
benchmark (sec): 0.0000000295

No crash, everything is OK
```
