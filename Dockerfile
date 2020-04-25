FROM ubuntu:18.04

ARG RUST_TOOLCHAIN="nightly"

ENV CARGO_HOME=/usr/local/rust
ENV RUSTUP_HOME=/usr/local/rust
ENV PATH="$PATH:$CARGO_HOME/bin"

# Update ubuntu
RUN apt-get update && apt-get upgrade -y
RUN apt-get install -y llvm curl cmake

# Install Rust and Cargo
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain "$RUST_TOOLCHAIN"


# Install honggfuzz-rs and subcommand in cargo
RUN apt-get install -y build-essential binutils-dev libunwind-dev libblocksruntime-dev
RUN cargo +nightly install --force honggfuzz

# Install cargo-fuzz (libfuzzer for Rust) and subcommand in cargo
RUN cargo +nightly install --force cargo-fuzz


# Install afl-rs and subcommand in cargo
RUN apt-get install -y build-essential libtool-bin python3 cmake automake bison libglib2.0-dev libpixman-1-dev clang python-setuptools
RUN cargo +nightly install --force afl

# create a new empty shell project
RUN USER=root cargo new --bin warf
WORKDIR /warf

# copy your source tree
COPY ./warf /warf

# Build the CLI tool
make build

ENTRYPOINT ["./warf"]