# Installation

Following installation steps are working for `Ubuntu 18.04`.

- Install Rust nightly
``` sh
# Install Rust and Cargo
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly
```

- Install system dependencies (Ubuntu/Debian):
``` sh
# Install LLVM
sudo apt install -y llvm curl

# Install honggfuzz-rs and subcommand in cargo
sudo apt install -y build-essential binutils-dev libunwind-dev libblocksruntime-dev
cargo +nightly install --force honggfuzz

# Install cargo-fuzz (libfuzzer for Rust) and subcommand in cargo
cargo +nightly install --force cargo-fuzz

# Install afl-rs and subcommand in cargo
sudo apt install -y build-essential libtool-bin python3 cmake automake bison libglib2.0-dev libpixman-1-dev clang python-setuptools llvm
cargo +nightly install --force afl
```

You're good to go now, build warf with `make build` ;)
