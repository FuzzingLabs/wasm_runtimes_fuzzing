# WARF CLI - TUTORIAL

The main goal of this tool is to provide an easy way to fuzz WebAssembly VMs/runtimes.
In addition to finding bugs, fuzzing will also generated samples/inputs that can be reused as unittest.


Main features are:
- Automatic fuzzing of runtimes harnesses (without any user interaction)
- Multiple fuzzing engines available (honggfuzz, afl, libfuzzer)
- Multi-threading (depending of the fuzzer, honggfuzz OK)
- Crash report/detection

## Available targets

Current target available can be listed with:
```sh
$ ./warf list-targets
parity_wasm_deserialize
[...]
wasmer_validate
```

# Subcommands

Help:
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
    list-targets     List all available targets
    target           Run one target with specific fuzzer
```

## Run targets

Help: `./warf target --help`.

Fuzzing with default fuzzing engine (honggfuzz):
``` sh
./warf target wasmer_validate
[...]
```

Fuzzing with other fuzzing engines:
``` sh
# --fuzzer <fuzzer>    Which fuzzer to run [default: Honggfuzz]  [possible values: Afl, Honggfuzz, Libfuzzer]
./warf target wasmer_validate --fuzzer afl
[...]
```

NOTES FOR AFL:

``` sh
# You need to execute the following commands to get afl running properly
echo core >/proc/sys/kernel/core_pattern
# sudo su -c "echo core >/proc/sys/kernel/core_pattern"
cd /sys/devices/system/cpu ; echo performance | tee cpu*/cpufreq/scaling_governor
# sudo su -c "cd /sys/devices/system/cpu; echo performance | tee cpu*/cpufreq/scaling_governor"
```

## Continuous fuzzing 

warf in continuous mode will stop after all target being executed once except if you're using infinite flag.

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
    -t, --timeout <timeout>    Set timeout per target [default: 10]
```

Prefered command:
``` sh
./warf continuously -i -t 600
# -i => infinite mode
# -t => timeout of 10 min, will restart the fuzzer every 10 min
```

## Fuzzer engines

It's possible to provide extra flags and environment variables to fuzzing engines (honggfuzz, afl++, libfuzzer).

- honggfuzz-rs [here](https://github.com/rust-fuzz/honggfuzz-rs#environment-variables)
- afl-rs [here](https://rust-fuzz.github.io/book/afl/tutorial.html)
- libfuzzer (cargo-fuzz) - [here](https://github.com/rust-fuzz/cargo-fuzz#usage)

# Future improvements for this tool

- Support new fuzzers (lain, fuzzcheck, customs, etc.)
- Compile all target before running fuzzing (no need to compile targets each time fuzzer restart)
