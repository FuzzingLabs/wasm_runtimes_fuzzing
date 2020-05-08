# How to analyze a crash

In this example, I will explain how to analyze a crash triggered by the fuzzer for `wasmer_compile_clif` target. 

# Where is my crash?

Crash are detected and annonced immediately by the fuzzer (like in the following `honggfuzz` interface):
``` sh
------------------------[  0 days 00 hrs 13 mins 37 secs ]----------------------
  Iterations : 265,459 [265.46k]
  Mode [3/3] : Feedback Driven Mode
      Target : hfuzz_target/x86_64-unknown-linux-gnu/release/wasmer_compile_clif
     Threads : 4, CPUs: 8, CPU%: 569% [71%/CPU]
       Speed : 58,231/sec [avg: 53,091]
==>     Crashes : 21 [unique: 1, blacklist: 0, verified: 0]
    Timeouts : 0 [10 sec]
 Corpus Size : 1,117, max: 8,192 bytes, init: 1,218 files
  Cov Update : 0 days 00 hrs 00 mins 00 secs ago
    Coverage : edge: 9,536/185,317 [5%] pc: 51 cmp: 167,998
---------------------------------- [ LOGS ] ------------------/ honggfuzz 2.0 /-
```

Crash has been stored by honggfuzz inside workspace/hfuzz/hfuzz_workspace/:
``` sh
$ ls workspace/hfuzz/hfuzz_workspace/wasmer_compile_clif

 HONGGFUZZ.REPORT.TXT
 input
'SIGABRT.PC.7ffff71dfe97.STACK.1b941138b2.CODE.-6.ADDR.0.INSTR.mov____0x108(%rsp),%rcx.fuzz'
```
Copy this file in `warf` and rename it to `crash_to_analyze.wasm`, will be easier for next part ;)


# Compile your target with warf debug

``` sh
$ ./warf debug wasmer_compile_clif

[...]
[WARF] Debug: debug_wasmer_compile_clif compiled
```
Your debugging tool is now available here: `./workspace/debug/target/debug/debug_wasmer_compile_clif`

# Run the debug tool with the crash

``` sh
$ ./debug_wasmer_compile_clif crash_to_analyze.wasm

Start debugging of wasmer_compile_clif
file_to_process: "crash_to_analyze.wasm"
thread 'main' panicked at 'index out of bounds: the len is 0 but the index is 0', /rustc/85976442558bf2d09cec3aa49c9c9ba86fb15c1f/src/libcore/slice/mod.rs:2791:10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
```
We can see here that it is a legitimate/valid bug triggering an index out of bounds panic in `wasmer`.

# Deeper analysis / BACKTRACE

You can use the `RUST_BACKTRACE` environment variable to get more information about the root cause of this bug:
``` sh
$ RUST_BACKTRACE=1 ./debug_wasmer_compile_clif crash_to_analyze.wasm

Start debugging of wasmer_compile_clif
file_to_process: "crash_to_analyze.wasm"
thread 'main' panicked at 'index out of bounds: the len is 0 but the index is 0', /rustc/85976442558bf2d09cec3aa49c9c9ba86fb15c1f/src/libcore/slice/mod.rs:2791:10
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:77
[...]
             at src/libcore/panicking.rs:85
  13: core::panicking::panic_bounds_check
             at src/libcore/panicking.rs:63
  14: <usize as core::slice::SliceIndex<[T]>>::index
             at /rustc/85976442558bf2d09cec3aa49c9c9ba86fb15c1f/src/libcore/slice/mod.rs:2791
  15: core::slice::<impl core::ops::index::Index<I> for [T]>::index
             at /rustc/85976442558bf2d09cec3aa49c9c9ba86fb15c1f/src/libcore/slice/mod.rs:2656
  16: <alloc::vec::Vec<T> as core::ops::index::Index<I>>::index
             at /rustc/85976442558bf2d09cec3aa49c9c9ba86fb15c1f/src/liballoc/vec.rs:1882
  17: <cranelift_entity::primary::PrimaryMap<K,V> as core::ops::index::Index<K>>::index
             at /home/scop/.cargo/registry/src/github.com-1ecc6299db9ec823/cranelift-entity-0.59.0/src/primary.rs:162
==>  18: wasmer_clif_fork_wasm::translation_utils::blocktype_params_results
==>             at /home/scop/.cargo/registry/src/github.com-1ecc6299db9ec823/wasmer-clif-fork-wasm-0.59.0/src/translation_utils.rs:194
==>  19: wasmer_clif_fork_wasm::code_translator::translate_operator
==>             at /home/scop/.cargo/registry/src/github.com-1ecc6299db9ec823/wasmer-clif-fork-wasm-0.59.0/src/code_translator.rs:185
  20: <wasmer_clif_backend::code::CraneliftFunctionCodeGenerator as wasmer_runtime_core::codegen::FunctionCodeGenerator<wasmer_clif_backend::code::CodegenError>>::feed_event
             at /home/scop/.cargo/registry/src/github.com-1ecc6299db9ec823/wasmer-clif-backend-0.16.2/src/code.rs:1208
  21: wasmer_runtime_core::codegen::MiddlewareChain::run
             at /home/scop/.cargo/registry/src/github.com-1ecc6299db9ec823/wasmer-runtime-core-0.16.2/src/codegen.rs:420
  22: wasmer_runtime_core::parse::read_module
             at /home/scop/.cargo/registry/src/github.com-1ecc6299db9ec823/wasmer-runtime-core-0.16.2/src/parse.rs:309
  23: <wasmer_runtime_core::codegen::StreamingCompiler<MCG,FCG,RM,E,CGEN> as wasmer_runtime_core::backend::Compiler>::compile
             at /home/scop/.cargo/registry/src/github.com-1ecc6299db9ec823/wasmer-runtime-core-0.16.2/src/codegen.rs:314
  24: wasmer_runtime_core::compile_with
             at /home/scop/.cargo/registry/src/github.com-1ecc6299db9ec823/wasmer-runtime-core-0.16.2/src/lib.rs:115
  25: wasmer_runtime::compile
             at /home/scop/.cargo/registry/src/github.com-1ecc6299db9ec823/wasmer-runtime-0.16.2/src/lib.rs:244
  26: fuzz_targets_common::fuzz_wasmer_compile_clif
             at common/src/lib.rs:30
[...]
             at src/libstd/rt.rs:51
  35: std::rt::lang_start
             at /rustc/85976442558bf2d09cec3aa49c9c9ba86fb15c1f/src/libstd/rt.rs:67
  36: main
  37: __libc_start_main
  38: _start
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

Congrats, you can now report your finding to the project owner like I've done [here](https://github.com/wasmerio/wasmer/issues/1372)
