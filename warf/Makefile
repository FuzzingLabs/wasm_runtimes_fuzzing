#
# Developer utils
#

# Run Rust fmt to make code cleaner
fmt:
	cargo fmt --all

# Check code quality with rust fmt
check-fmt:
	cargo fmt --all -- --check

# Clean generated fuzz target harnesses
clean:
	rm -rf fuzzer-afl/src/bin/*
	rm -rf fuzzer-honggfuzz/src/bin/*
	# todo for libfuzzer

# Clean generated fuzz target harnesses
clean-all: clean
	rm -rf target/
	rm -rf  fuzzer-honggfuzz/hfuzz_target/

#
# Testing utils
#
test-debug:
	./warf debug wasmparser_parser

test-libfuzzer:
	./warf debug wasmparser_parser --fuzzer libfuzzer

test-hfuzz:
	./warf target wasmparser_parser --fuzzer honggfuzz

test-afl:
	./warf target wasmparser_parser --fuzzer afl

# TODO - add more stuff here
test-all: build test-debug

#
# User utils
#

# Create an initial corpora of wasm modules
corpora:
	echo "TODO ../script/"

# Build the docker
docker:
	cd .. && docker build . -t warf

# Build the project locally
build:
	cargo +nightly build --release --out-dir=. -Z unstable-options