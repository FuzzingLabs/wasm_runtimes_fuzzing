#
# Developer utils
#

# Run Rust fmt to make code cleaner
fmt:
	cargo fmt --all

# Check code style quickly by running `rustfmt` over all code
check-fmt:
	cargo fmt --all -- --check

# Clean generated fuzz target harnesses
clean:
	rm -rf debug/src/bin/*
	rm -rf fuzzer-afl/src/bin/*
	rm -rf fuzzer-honggfuzz/src/bin/*
	# todo for libfuzzer

# Clean all generated fuzz target harnesses
clean-all: clean
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
# Testing warf cli sub-commands
#
test-debug: build
	./warf debug wasmparser_parser
	./workspace/debug/target/debug/debug_wasmparser_parser workspace/corpora/wasm/fib.wasm

test-continuously-all: build
	./warf continuously -t 2 --fuzzer libfuzzer
	./warf continuously -t 2 --fuzzer honggfuzz
	./warf continuously -t 2 --fuzzer afl

# TODO - test more stuff here
test-all: build test-debug test-continuously-all

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