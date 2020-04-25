#
# Developer utils
#

# Check code quality with rust fmt
fmt:
	cargo fmt --all -- --check


# Clean generated fuzz target harnesses
clean:
	rm -f fuzzer-afl/src/bin/*
	rm -f fuzzer-honggfuzz/src/bin/*

# Create an initial corpora of wasm modules
corpora:
	echo "TODO"

# Build the docker
docker:
	cd .. && docker build . -t warf

# Build the project locally
build:
	cargo +nightly build --out-dir=. -Z unstable-options