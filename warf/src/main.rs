extern crate regex;
extern crate structopt;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate failure;

// Strum contains all the trait definitions
extern crate strum;
#[macro_use]
extern crate strum_macros;

use crate::strum::IntoEnumIterator;
use failure::Error;
use structopt::StructOpt;

mod debug;
mod env;
mod exec_all;
mod fuzzers;
mod rust_fuzzers;
mod targets;
mod utils;

/// WARF - WebAssembly Runtimes Fuzzing project
#[derive(StructOpt, Debug)]
enum Cli {
    /// Run all fuzz targets
    #[structopt(name = "continuously")]
    Continuous {
        /// Only run target containing this string
        #[structopt(short = "q", long = "filter")]
        filter: Option<String>,
        /// Which fuzzer to run
        #[structopt(
            short = "f",
            long = "fuzzer",
            default_value = "Honggfuzz",
            raw(
                possible_values = "&fuzzers::Fuzzer::variants()",
                case_insensitive = "true"
            )
        )]
        fuzzer: fuzzers::Fuzzer,
        /// Set timeout per target
        #[structopt(short = "t", long = "timeout", default_value = "10")]
        timeout: i32,
        /// Set number of thread
        #[structopt(short = "n", long = "thread")]
        thread: Option<i32>,
        /// Set seed
        #[structopt(short = "s", long = "seed")]
        seed: Option<i32>,
        /// Set a compilation Sanitizer (advanced)
        #[structopt(
            long = "sanitizer",
            raw(
                possible_values = "&fuzzers::Sanitizer::variants()",
                case_insensitive = "true"
            )
        )]
        sanitizer: Option<fuzzers::Sanitizer>,
        // Run until the end of time (or Ctrl+C)
        #[structopt(short = "i", long = "infinite")]
        infinite: bool,
    },
    /// Run one target with specific fuzzer
    #[structopt(name = "target")]
    Run {
        /// Which target to run
        target: String,
        /// Which fuzzer to run
        #[structopt(
            short = "f",
            long = "fuzzer",
            default_value = "Honggfuzz",
            raw(
                possible_values = "&fuzzers::Fuzzer::variants()",
                case_insensitive = "true"
            )
        )]
        fuzzer: fuzzers::Fuzzer,
        /// Set timeout
        #[structopt(short = "t", long = "timeout")]
        timeout: Option<i32>,
        /// Set number of thread (only for hfuzz)
        #[structopt(short = "n", long = "thread")]
        thread: Option<i32>,
        /// Set seed
        #[structopt(short = "s", long = "seed")]
        seed: Option<i32>,
        /// Set dictionary file
        #[structopt(short = "d", long = "dict")]
        dict: Option<String>,
        /// Set a compilation Sanitizer (advanced)
        #[structopt(
            long = "sanitizer",
            raw(
                possible_values = "&fuzzers::Sanitizer::variants()",
                case_insensitive = "true"
            )
        )]
        sanitizer: Option<fuzzers::Sanitizer>,
    },
    /// Debug one target
    #[structopt(name = "debug")]
    Debug {
        /// Which target to debug
        target: String,
    },
    /// List all available targets
    #[structopt(name = "list")]
    ListTargets,
    /// Run WebAssembly module on all targets
    #[structopt(name = "execute-all")]
    ExecuteAll {
        /// Which wasm to execute
        wasm: String,
    },
    /// Run WebAssembly module on all targets with benchmark
    #[structopt(name = "benchmark-all")]
    BenchmarkAll {
        /// Which wasm to execute
        wasm: String,
    },
}

/// Main function catching errors
fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        for cause in e.iter_chain().skip(1) {
            eprintln!("caused by: {}", cause);
        }
        ::std::process::exit(1);
    }
}

/// Parsing of CLI arguments
fn run() -> Result<(), Error> {
    use Cli::*;
    let cli = Cli::from_args();

    match cli {
        ExecuteAll { wasm } => {
            exec_all::run_exec_all(wasm, false)?;
        }
        BenchmarkAll { wasm } => {
            exec_all::run_exec_all(wasm, true)?;
        }
        // list all targets
        ListTargets => {
            list_targets()?;
        }
        // Fuzz one target
        Run {
            target,
            fuzzer,
            timeout,
            thread,
            seed,
            dict,
            sanitizer,
        } => {
            let config = fuzzers::FuzzerConfig {
                timeout,
                thread,
                sanitizer,
                seed,
                dict,
            };
            run_target(target, fuzzer, config)?;
        }
        // Debug one target
        Debug { target } => {
            debug::run_debug(target)?;
        }
        // Fuzz multiple targets
        Continuous {
            filter,
            timeout,
            fuzzer,
            thread,
            seed,
            sanitizer,
            infinite,
        } => {
            let config = fuzzers::FuzzerConfig {
                timeout: Some(timeout),
                thread,
                sanitizer,
                seed,
                dict: None,
            };
            run_continuously(filter, fuzzer, config, infinite)?;
        }
    }
    Ok(())
}

/// List all targets available
fn list_targets() -> Result<(), Error> {
    for target in targets::get_targets() {
        println!("{}", target);
    }
    Ok(())
}

/// Run fuzzing on only one target
fn run_target(
    target: String,
    fuzzer: fuzzers::Fuzzer,
    config: fuzzers::FuzzerConfig,
) -> Result<(), Error> {
    let target = match targets::Targets::iter().find(|x| x.name() == target) {
        None => bail!(
            "Don't know target `{}`. {}",
            target,
            if let Some(alt) = utils::did_you_mean(&target, &targets::get_targets()) {
                format!("Did you mean `{}`?", alt)
            } else {
                "".into()
            }
        ),
        Some(t) => t,
    };

    use fuzzers::Fuzzer::*;
    match fuzzer {
        Afl => {
            let afl = rust_fuzzers::FuzzerAfl::new(config)?;
            afl.run(target)?;
        }
        Honggfuzz => {
            let hfuzz = rust_fuzzers::FuzzerHfuzz::new(config)?;
            hfuzz.run(target)?;
        }
        Libfuzzer => {
            let lfuzz = rust_fuzzers::FuzzerLibfuzzer::new(config)?;
            lfuzz.run(target)?;
        }
    }
    Ok(())
}

/// Run fuzzing on multiple target matching the filter option
fn run_continuously(
    filter: Option<String>,
    fuzzer: fuzzers::Fuzzer,
    config: fuzzers::FuzzerConfig,
    infinite: bool,
) -> Result<(), Error> {
    let run = |target: &str| -> Result<(), Error> {
        let target = match targets::Targets::iter().find(|x| x.name() == target) {
            None => bail!(
                "Don't know target `{}`. {}",
                target,
                if let Some(alt) = utils::did_you_mean(&target, &targets::get_targets()) {
                    format!("Did you mean `{}`?", alt)
                } else {
                    "".into()
                }
            ),
            Some(t) => t,
        };

        use fuzzers::Fuzzer::*;
        match fuzzer {
            Afl => {
                let hfuzz = rust_fuzzers::FuzzerAfl::new(config.clone())?;
                hfuzz.run(target)?;
            }
            Honggfuzz => {
                let hfuzz = rust_fuzzers::FuzzerHfuzz::new(config.clone())?;
                hfuzz.run(target)?;
            }
            Libfuzzer => {
                let hfuzz = rust_fuzzers::FuzzerLibfuzzer::new(config.clone())?;
                hfuzz.run(target)?;
            }
        }
        Ok(())
    };

    let targets = targets::get_targets();
    let targets = targets
        .iter()
        .filter(|x| filter.as_ref().map(|f| x.contains(f)).unwrap_or(true));

    'cycle: loop {
        'targets_pass: for target in targets.clone() {
            if let Err(e) = run(target) {
                match e.downcast::<fuzzers::FuzzerQuit>() {
                    Ok(_) => {
                        println!("Fuzzer failed so we'll continue with the next one");
                        continue 'targets_pass;
                    }
                    Err(other_error) => return Err(other_error),
                }
            }
        }

        if !infinite {
            break 'cycle;
        }
    }
    Ok(())
}
