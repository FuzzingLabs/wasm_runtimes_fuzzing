#
# User utils
#

# Build the project locally
build:
	cargo +nightly build --release --out-dir=. -Z unstable-options

# Build the docker
docker:
	cd .. && docker build . -t warf

# Create an initial corpora of wasm modules
corpora:
	echo "TODO ../script/"

#
# Developer utils
#

# Run Rust fmt to make code cleaner
fmt:
	cargo fmt --all

# Clean only warf target
clean:
	rm -rf target/

# Clean all generated fuzz target harnesses
clean-all:
	rm -rf target/
	rm -rf workspace/afl/target/
	rm -rf workspace/hfuzz/hfuzz_target/
	rm -rf workspace/libfuzzer/fuzz/target/

# Update rust fuzzers (afl, honggfuzz and libfuzzer) to last version
update:
	cargo +nightly install --force honggfuzz
	cargo +nightly install --force cargo-fuzz
	cargo +nightly install --force afl

#
# Testing utils
# warf cli sub-commands
#

# Compile one debug tool and run a simple wasm to it
test-debug: build
	./warf debug wasmparser_parser
	./workspace/debug/target/debug/debug_wasmparser_parser workspace/corpora/wasm/fib.wasm

# Run all fuzzing hardness over libfuzzer
test-libfuzzer: build
	./warf continuously -t 2 --fuzzer libfuzzer

# Run all fuzzing hardness over honggfuzz
test-honggfuzz: build
	./warf continuously -t 2 --fuzzer honggfuzz

# Run all fuzzing hardness over afl
test-afl: build
	./warf continuously -t 2 --fuzzer afl

# Run all tests
# (Really) Heavy test
test-fuzzers: test-libfuzzer test-honggfuzz test-afl

# TODO - test more stuff here
test-all: test-debug test-fuzzers


#
# CI utils
#

# Check code style quickly by running `rustfmt` over all code
check-fmt:
	cargo fmt --all -- --check