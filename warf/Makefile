.PHONY: default update check-fmt help build docker corpora fmt clean clean-all test test-bench test-debug test-libfuzzer test-honggfuzz test-afl test-all

#
# User utils
#

default: build

help:
	@echo 'Management commands for warf'
	@echo
	@echo 'Usage:'
	@echo '    make build 										Compile the project locally.'
	@echo '    make docker										Build a docker image for this project.'
	@echo '    make corpora		 								TODO'
	@echo
	@echo '    make fmt 										Run Rust fmt.'
	@echo '    make clean 										Clean only warf binary.'
	@echo '    make clean-all 									Clean all (warf && compiled fuzz target harnesses).'
	@echo
	@echo '    make test										Simple test to check warf and execute_all is working.'
	@echo '    make test-bench									Simple benchmark using execute_all.'
	@echo '    make test-debug									Test running a simple wasm to a debugging tool.'
	@echo '    make test-{libfuzzer, honggfuzz, afl}			Test one fuzzing hardness over choosen fuzzer.'
	@echo '    make test-continuously-{libfuzzer, hfuzz, afl}	Test all fuzzing hardness over choosen fuzzer.'
	@echo '    make test-all		 							Test all fuzzing hardness over all fuzzers.'
	@echo

# Compile the project locally
build:
	cargo +nightly build --release --out-dir=. -Z unstable-options

# Build the docker
# NOTE: `make docker CACHE=--no-cache` to force docker update/build everything again
docker:
	DOCKER_BUILDKIT=1 docker build . -t warf $(CACHE)
	@echo 'USAGE: docker run -it -v `pwd`/workspace:/warf/workspace warf list'

# Create an initial corpora of wasm modules
corpora:
	echo "TODO"

# Run Rust fmt to make code cleaner
fmt:
	cargo fmt --all

# Clean only warf target
clean:
	rm -rf target/
	rm -rf ./warf

# Clean all (warf && compiled fuzz target harnesses).
clean-all: clean
	rm -rf workspace/debug/target
	rm -rf workspace/execute_all/target
	rm -rf workspace/afl/target/
	rm -rf workspace/hfuzz/hfuzz_target/
	rm -rf workspace/libfuzzer/fuzz/target/

# update to latest cargo package version
update:
	cargo update
	cd targets && cargo update
	cd execute_all && cargo update
	cd debug && cargo update
	(cd workspace/afl && cargo update) || true
	(cd workspace/hfuzz && cargo update) || true
	(cd workspace/libfuzzer && cargo update) || true

#
# Testing utils
# warf cli sub-commands
#

# Simple test using docker.
docker-test: docker
	docker run -it -v `pwd`/workspace:/warf/workspace warf list

# Simple test with wasm using docker.
docker-test-all: docker
	docker run -it -v `pwd`/workspace:/warf/workspace warf execute-all workspace/corpora/wasm/fib.wasm

# Simple test using execute_all 
test: build
	./warf execute-all workspace/corpora/wasm/fib.wasm

# Simple benchmark test using execute_all 
test-bench: build
	./warf benchmark-all workspace/corpora/wasm/fib.wasm

# Compile one debug tool and run a simple wasm to it
test-debug: build
	./warf debug wasmparser_parser
	./workspace/debug/target/debug/debug_wasmparser_parser workspace/corpora/wasm/fib.wasm

# Run one fuzzing hardness over libfuzzer during 2s
test-libfuzzer: build
	./warf target wasmparser_parser -t 2 -s 1234 --fuzzer libfuzzer

# Run fuzzing hardness over honggfuzz during 2s
test-honggfuzz: build
	./warf target wasmparser_parser -t 2 -s 1234 --fuzzer honggfuzz

# Run fuzzing hardness over afl during 2s
test-afl: build
	AFL_SKIP_CPUFREQ=1 \
	AFL_I_DONT_CARE_ABOUT_MISSING_CRASHES=1 \
	./warf target wasmparser_parser -t 2 -s 1234 --fuzzer afl


# Run all fuzzing hardness over libfuzzer
test-continuously-libfuzzer: build
	./warf continuously -t 2 -s 1234 --fuzzer libfuzzer

# Run all fuzzing hardness over honggfuzz
test-continuously-honggfuzz: build
	./warf continuously -t 2 -s 1234 --fuzzer honggfuzz

# Run all fuzzing hardness over afl
test-continuously-afl: build
	AFL_SKIP_CPUFREQ=1 \
	AFL_I_DONT_CARE_ABOUT_MISSING_CRASHES=1 \
	./warf continuously -t 2 -s 1234 --fuzzer afl

# Run all tests
# (Really) Heavy test
test-all: test test-bench test-debug test-libfuzzer test-honggfuzz test-afl

#
# Developer utils
#

# Update rust fuzzers (afl, honggfuzz and libfuzzer) to last version
# TODO: move to an install/update script?
update-fuzzer:
	cargo +nightly install --force honggfuzz
	cargo +nightly install --force cargo-fuzz
	cargo +nightly install --force afl

#
# CI utils
#

# Check code style quickly by running `rustfmt` over all code
check-fmt:
	cargo fmt --all -- --check

build-exec-all: build
	./warf execute-all
