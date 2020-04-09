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
$ ./target/debug/warf list-targets
parity_wasm_deserialize
wasmer_validate
```

# Subcommands

Help:
``` sh
$ ./target/debug/warf help
[...]
SUBCOMMANDS:
    continuously    Run all fuzz targets
    debug           Debug one target
    help            Prints this message or the help of the given subcommand(s)
    list-targets    List all available targets
    target          Run one target with specific fuzzer
```

## Run targets

Help: `./target/debug/warf target --help`.

Run targets: `./target/debug/warf target wasmer_validate`.
Using other fuzzing engines:
``` sh
# --fuzzer <fuzzer>    Which fuzzer to run [default: Honggfuzz]  [possible values: Afl, Honggfuzz, Libfuzzer]
./target/debug/warf target wasmer_validate --fuzzer afl
```

## Continuous fuzzing 

CAUTIONS: all_fuzz continuous mode will stop after all target being executed once if you are not providing infinite flag.

Help:
``` sh
$ ./target/debug/warf continuously --help
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
./target/debug/warf continuously -i wasmer -t 600
# -i => infinite mode
# -q => will run all wasmer targets
# -t => timeout of 10 min, will restart the fuzzer every 10 min
```

## fuzzer engines

It's possible to provide extra flags to fuzzing engines (honggfuzz, afl, libfuzzer)

### honggfuzz-rs (WORKING)

FLAG: `HFUZZ_RUN_ARGS`

Limit corpus file size: `HFUZZ_RUN_ARGS="-F 500000"`.
TODO

### afl-rs (ALMOST WORKING)


- You need to execute the following commands to get afl running properly:
``` sh
echo core >/proc/sys/kernel/core_pattern
# sudo su -c "echo core >/proc/sys/kernel/core_pattern"
cd /sys/devices/system/cpu ; echo performance | tee cpu*/cpufreq/scaling_governor
# sudo su -c "cd /sys/devices/system/cpu; echo performance | tee cpu*/cpufreq/scaling_governor"
```


### libfuzzer (NOT WORKING)

TODO

# Improvements

This tool will be improved over the time

## General improvement for this tool

- add first time running script for afl
- add more documentation
- fix libfuzzer (cargofuzz) => maybe use cargo-fuzz instead of existing code.
- support new fuzzers (lain, fuzzcheck, customs, etc.)
- compile all target before running fuzzing (no need to compile targets each time fuzzer restart) ?
- Verify sharing coverage + seeds work as expected
