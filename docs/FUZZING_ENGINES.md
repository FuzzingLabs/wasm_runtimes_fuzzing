# FUZZING ENGINES

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

