# warf advanced options

When using the `target` and `continuously` subcommands, you will have access to a bench of additionnal options.

```
$ ./warf target --help
USAGE:
    warf target [OPTIONS] <target>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --dict <dict>              Set dictionary file
    -f, --fuzzer <fuzzer>          Which fuzzer to run [default: Honggfuzz]  [possible values: Afl, Honggfuzz,
                                   Libfuzzer]
        --sanitizer <sanitizer>    Set a compilation Sanitizer (advanced) [possible values: Address, Leak, Memory,
                                   Thread]
    -s, --seed <seed>              Set seed
    -n, --thread <thread>          Set number of thread (only for hfuzz)
    -t, --timeout <timeout>        Set timeout

ARGS:
    <target>    Which target to run
```

## dict (only for `target`)

Allow you to provide dictionaries with input language keywords or other interesting byte sequences. More information about libfuzzer dictionary format [here](http://llvm.org/docs/LibFuzzer.html#dictionaries)

Example:
``` sh
$ ./warf target wast_parser -d dictionary/wast.dict
```

## fuzzer

Allow you to change which fuzzing engines will be used. The default/recommended one is honggfuzz because it's supporting all the features (especially multithreading)

Example:
``` sh
$ ./warf target wast_parser -f Libfuzzer
```

## sanitizer

Allow you to compile your targets with sanitizer. Sanitizer help finding bugs but slowdown your fuzzing speed. More information about sanitizer [here](https://github.com/google/sanitizers) 

Example:
``` sh
$ ./warf target wast_parser --sanitizer address
```

## seed

Allow you to make fuzzing deterministic and reproductible.

Example:
``` sh
$ ./warf target wast_parser -s 12345
```

## thread

Allow you to activate multithreading and run multiple fuzzing process at the same time. By default, honggfuzz will do it using your number of core divide by 2.

Example:
``` sh
$ ./warf target wast_parser -n 3 # use 3 cores
```


## timeout

Allow you to limit the amount of time per fuzzing sessions. This option is useful if you want to fuzz multiple targets with a limited time for each.

Example:
``` sh
$ ./warf continuously -t 3600 # seconds i.e. 1 hour
```


# Other notes


## fuzzers environment variables

It's possible to provide extra flags and environment variables to fuzzing engines (honggfuzz, afl++, libfuzzer).

- honggfuzz-rs [here](https://github.com/rust-fuzz/honggfuzz-rs#environment-variables)
- afl-rs [here](https://rust-fuzz.github.io/book/afl/tutorial.html)
- libfuzzer (cargo-fuzz) - [here](https://github.com/rust-fuzz/cargo-fuzz#usage)


## afl-rs 

NOTES FOR AFL:

``` sh
# You need to execute the following commands to get afl running properly
echo core >/proc/sys/kernel/core_pattern
# sudo su -c "echo core >/proc/sys/kernel/core_pattern"
cd /sys/devices/system/cpu ; echo performance | tee cpu*/cpufreq/scaling_governor
# sudo su -c "cd /sys/devices/system/cpu; echo performance | tee cpu*/cpufreq/scaling_governor"
```

