FROM ubuntu:18.04

ARG RUST_TOOLCHAIN="nightly"

ENV CARGO_HOME=/usr/local/rust
ENV RUSTUP_HOME=/usr/local/rust
ENV PATH="$PATH:$CARGO_HOME/bin"

# Update ubuntu
RUN apt-get update && \
	apt-get install -y \
		llvm \
		curl \
		build-essential \
		binutils-dev \
		libunwind-dev \
		libblocksruntime-dev \
		libtool-bin \
		python3 \
		cmake automake \
		bison libglib2.0-dev \
		libpixman-1-dev clang \
		python-setuptools

# Install Rust and Cargo
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain "$RUST_TOOLCHAIN"

# Install honggfuzz-rs and subcommand in cargo
RUN cargo +nightly install --force honggfuzz
# Install cargo-fuzz (libfuzzer for Rust) and subcommand in cargo
RUN cargo +nightly install --force cargo-fuzz
# Install afl-rs and subcommand in cargo
RUN cargo +nightly install --force afl

# create a new empty shell project
RUN USER=root cargo new --bin warf
WORKDIR /warf

# copy over your manifests
# COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build
RUN rm src/*.rs

# Copy all
COPY . .

# Build the CLI tool
RUN make build

ENTRYPOINT ["./warf"]