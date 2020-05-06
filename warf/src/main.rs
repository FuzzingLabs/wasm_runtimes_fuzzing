extern crate structopt;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate failure;
extern crate fs_extra;
extern crate regex;

use failure::{Error};

use structopt::StructOpt;

mod fuzzers;

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
            long = "fuzzer",
            default_value = "Honggfuzz",
            raw(possible_values = "&fuzzers::Fuzzer::variants()", case_insensitive = "true")
        )]
        fuzzer: fuzzers::Fuzzer,
        /// Set timeout per target
        #[structopt(short = "t", long = "timeout", default_value = "10")]
        timeout: i32,
        /// Set number of thread (only for hfuzz)
        #[structopt(short = "n", long = "thread")]
        thread: Option<i32>,
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
            long = "fuzzer",
            default_value = "Honggfuzz",
            raw(possible_values = "&fuzzers::Fuzzer::variants()", case_insensitive = "true")
        )]
        fuzzer: fuzzers::Fuzzer,
        /// Set timeout per target
        #[structopt(short = "t", long = "timeout")]
        timeout: Option<i32>,
        /// Set number of thread (only for hfuzz)
        #[structopt(short = "n", long = "thread")]
        thread: Option<i32>,
    },
    /// Build all targets for this specific fuzzer
    #[structopt(name = "build")]
    Build {
        /// Which fuzzer to run
        #[structopt(
            long = "fuzzer",
            default_value = "Honggfuzz",
            raw(possible_values = "&fuzzers::Fuzzer::variants()", case_insensitive = "true")
        )]
        fuzzer: fuzzers::Fuzzer,
    },
    /// Debug one target
    #[structopt(name = "debug")]
    Debug {
        /// Which target to debug
        target: String,
    },
    /// List all available targets
    #[structopt(name = "list-targets")]
    ListTargets,
    /// Run WebAssembly module on all targets
    #[structopt(name = "execute-all")]
    ExecuteAll,
    /// Run WebAssembly module on all targets with benchmark
    #[structopt(name = "benchmark-all")]
    BenchmarkAll,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        for cause in e.iter_chain().skip(1) {
            eprintln!("caused by: {}", cause);
        }
        ::std::process::exit(1);
    }
}

fn run() -> Result<(), Error> {
    use Cli::*;
    let cli = Cli::from_args();

    match cli {
        ExecuteAll => {
            fuzzers::run_exec_all(false)?;
        }
        BenchmarkAll => {
            fuzzers::run_exec_all(true)?;
        }
        ListTargets => {
            list_targets()?;
        }
        Build { fuzzer } => {
            use fuzzers::Fuzzer::*;
            match fuzzer {
                Afl => fuzzers::build_targets_afl()?,
                Honggfuzz => fuzzers::build_honggfuzz()?,
                Libfuzzer => fuzzers::build_libfuzzer()?,
            }
        }
        Run {
            target,
            fuzzer,
            timeout,
            thread,
        } => {
            run_target(&target, fuzzer, timeout, thread)?;
        }
        Debug { target } => {
            let targets = fuzzers::get_targets()?;
            if targets.iter().find(|x| *x == &target).is_none() {
                bail!(
                    "Don't know target `{}`. {}",
                    target,
                    if let Some(alt) = did_you_mean(&target, &targets) {
                        format!("Did you mean `{}`?", alt)
                    } else {
                        "".into()
                    }
                );
            }

            fuzzers::run_debug(&target)?;
        }
        Continuous {
            filter,
            timeout,
            fuzzer,
            thread,
            infinite,
        } => {
            run_continuously(filter, fuzzer, Some(timeout), thread, infinite)?;
        }
    }
    Ok(())
}

fn list_targets() -> Result<(), Error> {
    for target in &fuzzers::get_targets()? {
        println!("{}", target);
    }
    Ok(())
}

fn run_target(
    target: &str,
    fuzzer: fuzzers::Fuzzer,
    timeout: Option<i32>,
    thread: Option<i32>,
) -> Result<(), Error> {
    let targets = fuzzers::get_targets()?;
    if targets.iter().find(|x| *x == &target).is_none() {
        bail!(
            "Don't know target `{}`. {}",
            target,
            if let Some(alt) = did_you_mean(&target, &targets) {
                format!("Did you mean `{}`?", alt)
            } else {
                "".into()
            }
        );
    }

    use fuzzers::Fuzzer::*;
    match fuzzer {
        Afl => fuzzers::run_afl(&target, timeout, None)?, // TODO - fix thread
        Honggfuzz => fuzzers::run_honggfuzz(&target, timeout, thread)?,
        Libfuzzer => fuzzers::run_libfuzzer(&target, timeout, None)?, // TODO - fix thread
    }
    Ok(())
}

fn run_continuously(
    filter: Option<String>,
    fuzzer: fuzzers::Fuzzer,
    timeout: Option<i32>,
    thread: Option<i32>,
    infinite: bool,
) -> Result<(), Error> {
    let run = |target: &str| -> Result<(), Error> {
        use fuzzers::Fuzzer::*;
        match fuzzer {
            Afl => fuzzers::run_afl(&target, timeout, None)?, // TODO - fix thread
            Honggfuzz => fuzzers::run_honggfuzz(&target, timeout, thread)?,
            Libfuzzer => fuzzers::run_libfuzzer(&target, timeout, None)?, // TODO - fix thread
        }
        Ok(())
    };

    let targets = fuzzers::get_targets()?;
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
                    Err(other_error) => Err(other_error)?,
                }
            }
        }

        if !infinite {
            break 'cycle;
        }
    }

    Ok(())
}

/// Produces a string from a given list of possible values which is similar to
/// the passed in value `v` with a certain confidence.
/// Thus in a list of possible values like ["foo", "bar"], the value "fop" will yield
/// `Some("foo")`, whereas "blark" would yield `None`.
///
/// Originally from [clap] which is Copyright (c) 2015-2016 Kevin B. Knapp
///
/// [clap]: https://github.com/kbknapp/clap-rs/blob/dc7ae65fb784dc355d56f09554f1216b22755c3e/src/suggestions.rs
pub fn did_you_mean<'a, T: ?Sized, I>(v: &str, possible_values: I) -> Option<&'a str>
where
    T: AsRef<str> + 'a,
    I: IntoIterator<Item = &'a T>,
{
    extern crate strsim;

    let mut candidate: Option<(f64, &str)> = None;
    for pv in possible_values {
        let confidence = strsim::jaro_winkler(v, pv.as_ref());
        if confidence > 0.8 && (candidate.is_none() || (candidate.as_ref().unwrap().0 < confidence))
        {
            candidate = Some((confidence, pv.as_ref()));
        }
    }
    match candidate {
        None => None,
        Some((_, candidate)) => Some(candidate),
    }
}
