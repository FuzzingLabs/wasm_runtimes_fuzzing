#![allow(deprecated)]

extern crate structopt;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate failure;
extern crate regex;

use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use failure::{Error, ResultExt};
use regex::Regex;
use structopt::StructOpt;

/// WARF - WebAssembly Runtimes Fuzzing project
#[derive(StructOpt, Debug)]
enum Cli {
    /// Run all fuzz targets
    #[structopt(name = "continuously")]
    Continuous {
        /// Only run target containing this string
        #[structopt(short = "q", long = "filter")]
        filter: Option<String>,
        /// Set timeout per target
        #[structopt(short = "t", long = "timeout", default_value = "10")]
        timeout: i32,
        // Run until the end of time (or Ctrl+C)
        #[structopt(short = "i", long = "infinite")]
        infinite: bool,
        /// Which fuzzer to run
        #[structopt(
            long = "fuzzer",
            default_value = "Honggfuzz",
            raw(possible_values = "&Fuzzer::variants()", case_insensitive = "true")
        )]
        fuzzer: Fuzzer,
        // Run `cargo update` between cycles
        #[structopt(long = "cargo-update")]
        cargo_update: bool,
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
            raw(possible_values = "&Fuzzer::variants()", case_insensitive = "true")
        )]
        fuzzer: Fuzzer,
    },
    /// Build all targets for this specific fuzzer
    #[structopt(name = "build")]
    Build {
        /// Which fuzzer to run
        #[structopt(
            long = "fuzzer",
            default_value = "Honggfuzz",
            raw(possible_values = "&Fuzzer::variants()", case_insensitive = "true")
        )]
        fuzzer: Fuzzer,
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
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        for cause in e.causes().skip(1) {
            eprintln!("caused by: {}", cause);
        }
        ::std::process::exit(1);
    }
}

fn run() -> Result<(), Error> {
    use Cli::*;
    let cli = Cli::from_args();

    match cli {
        ListTargets => {
            for target in &get_targets()? {
                println!("{}", target);
            }
        }
        Build { fuzzer } => {
            use Fuzzer::*;
            match fuzzer {
                Afl => build_targets_afl()?,
                Honggfuzz => build_honggfuzz()?,
                Libfuzzer => build_libfuzzer()?,
            }
        }
        Run { target, fuzzer } => {
            let targets = get_targets()?;
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

            use Fuzzer::*;
            match fuzzer {
                Afl => run_afl(&target, None)?,
                Honggfuzz => run_honggfuzz(&target, None)?,
                Libfuzzer => run_libfuzzer(&target, None)?,
            }
        }
        Debug { target } => {
            let targets = get_targets()?;
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

            run_debug(&target)?;
        }
        Continuous {
            filter,
            timeout,
            infinite,
            fuzzer,
            cargo_update,
        } => {
            let run = |target: &str| -> Result<(), Error> {
                use Fuzzer::*;
                match fuzzer {
                    Afl => run_afl(&target, Some(timeout))?,
                    Honggfuzz => run_honggfuzz(&target, Some(timeout))?,
                    Libfuzzer => run_libfuzzer(&target, Some(timeout))?,
                }
                Ok(())
            };

            let targets = get_targets()?;
            let targets = targets
                .iter()
                .filter(|x| filter.as_ref().map(|f| x.contains(f)).unwrap_or(true));

            'cycle: loop {
                'targets_pass: for target in targets.clone() {
                    if let Err(e) = run(target) {
                        match e.downcast::<FuzzerQuit>() {
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

                if cargo_update {
                    run_cargo_update()?;
                }
            }
        }
    }
    Ok(())
}

fn default_dir() -> Result<PathBuf, Error> {
    let p = env::var("CARGO_MANIFEST_DIR")
        .map(From::from)
        .or_else(|_| env::current_dir())?;
    Ok(p)
}

fn targets_dir() -> Result<PathBuf, Error> {
    let p = default_dir()?.join("targets");

    Ok(p)
}

fn corpora_dir() -> Result<PathBuf, Error> {
    let p = default_dir()?.join("workspace").join("corpora");

    Ok(p)
}

fn create_wasm_dir() -> Result<PathBuf, Error> {
    let seed_dir = corpora_dir()?.join("wasm");
    fs::create_dir_all(&seed_dir).context(format!("unable to create corpora/wasm dir"))?;
    Ok(seed_dir)
}

fn get_targets() -> Result<Vec<String>, Error> {
    let source = targets_dir()?.join("src/lib.rs");
    let targets_rs = fs::read_to_string(&source).context(format!("unable to read {:?}", source))?;
    let match_fuzz_fs = Regex::new(r"pub fn fuzz_(\w+)\(")?;
    let target_names = match_fuzz_fs
        .captures_iter(&targets_rs)
        .map(|x| x[1].to_string());
    Ok(target_names.collect())
}

fn run_cargo_update() -> Result<(), Error> {
    let run = Command::new("cargo")
        .arg("update")
        .spawn()
        .context("error starting `cargo update`")?
        .wait()
        .context("error running `cargo update`")?;

    ensure!(
        run.success(),
        "error running `cargo update`: Exited with {:?}",
        run.code()
    );
    Ok(())
}

#[derive(Fail, Debug)]
#[fail(display = "[WARF] Fuzzer quit")]
pub struct FuzzerQuit;

/// Build all targets with honggfuzz
fn build_honggfuzz() -> Result<(), Error> {
    let fuzzer = Fuzzer::Honggfuzz;

    for target in &get_targets()? {
        write_fuzzer_target(fuzzer, target)?;
    }
    let dir = fuzzer.dir()?;

    println!("[WARF] {}: Start building", fuzzer);

    // Build fuzzing target
    let fuzzer_bin = Command::new("cargo")
        .args(&["hfuzz", "build"])
        .current_dir(&dir)
        .spawn()
        .context(format!("error building {} targets", fuzzer))?
        .wait()
        .context(format!("error while waiting for {:?} building", fuzzer))?;

    // Check if success
    if !fuzzer_bin.success() {
        Err(FuzzerQuit)?;
    }
    println!("[WARF] {}: building OK", fuzzer);
    Ok(())
}

fn run_honggfuzz(target: &str, timeout: Option<i32>) -> Result<(), Error> {
    let fuzzer = Fuzzer::Honggfuzz;
    write_fuzzer_target(fuzzer, target)?;
    let dir = fuzzer.dir()?;
    let work_dir = fuzzer.work_dir()?;

    //let seed_dir = create_seed_dir(&target)?;
    let seed_dir = create_wasm_dir()?;

    // test if this is the first time we run the fuzzer
    // and if existing coverage file exist
    // mut input_folder = format!("fuzzer-honggfuzz/hfuzz_workspace/{}/input", target);
    let mut input_folder = seed_dir.to_string_lossy().to_string();
    if Path::new(&work_dir).exists() {
        println!("{:?}", work_dir);
        input_folder = work_dir.to_string_lossy().to_string();
    } else {
        // create workir directory
        fs::create_dir_all(&work_dir).context(format!("unable to create corpora/wasm dir"))?;
    }

    let args = format!(
        "-i {} \
         --output {} \
         {} \
         {}",
        input_folder,
        work_dir.to_string_lossy().to_string(),
        if let Some(t) = timeout {
            format!("--run_time {}", t)
        } else {
            "".into()
        },
        env::var("HFUZZ_RUN_ARGS").unwrap_or_default()
    );

    //.env("RUSTFLAGS", "-C debuginfo=2")

    // Honggfuzz will first build than run the fuzzer using cargo
    let fuzzer_bin = Command::new("cargo")
        .args(&["hfuzz", "run", &target])
        .env("HFUZZ_RUN_ARGS", &args)
        .current_dir(&dir)
        .spawn()
        .context(format!("error starting {:?} to run {}", fuzzer, target))?
        .wait()
        .context(format!(
            "error while waiting for {:?} running {}",
            fuzzer, target
        ))?;

    if !fuzzer_bin.success() {
        Err(FuzzerQuit)?;
    }
    Ok(())
}

/// Build all targets with afl
fn build_targets_afl() -> Result<(), Error> {
    for target in &get_targets()? {
        build_afl(target)?;
    }
    Ok(())
}

/// Build single target with afl
fn build_afl(target: &str) -> Result<(), Error> {
    let fuzzer = Fuzzer::Afl;
    write_fuzzer_target(fuzzer, target)?;

    let dir = fuzzer.dir()?;

    let build_cmd = Command::new("cargo")
        .args(&["afl", "build", "--bin", target]) // TODO: not sure we want to compile afl in "--release"
        .current_dir(&dir)
        .spawn()
        .context(format!(
            "error starting build for {:?} of {}",
            fuzzer, target
        ))?
        .wait()
        .context(format!(
            "error while waiting for build for {:?} of {}",
            fuzzer, target
        ))?;

    if !build_cmd.success() {
        Err(FuzzerQuit)?;
    }

    Ok(())
}

fn run_afl(target: &str, _timeout: Option<i32>) -> Result<(), Error> {
    let fuzzer = Fuzzer::Afl;

    // Build the target if target not already compiled
    if !default_dir()?
        .join(&format!("target/debug/{}", target))
        .exists()
    {
        println!(
            "[WARF] {}: {:?} don't exist",
            fuzzer,
            default_dir()?.join(&format!("target/debug/{}", target))
        );
        build_afl(target)?;
    }

    let dir = fuzzer.dir()?;

    let seed_dir = create_wasm_dir()?;
    let corpus_dir = fuzzer.work_dir()?; //create_corpus_dir(&dir, target)?;
                                         // create corpus dir (workspace/afl)
    fs::create_dir_all(&corpus_dir).context(format!("unable to create corpora/wasm dir"))?;

    // Determined if existing fuzzing session exist
    let queue_dir = corpus_dir.join("queue");
    let input_arg: &OsStr = if queue_dir.is_dir() && fs::read_dir(queue_dir)?.next().is_some() {
        "-".as_ref()
    } else {
        seed_dir.as_ref()
    };

    // Run the fuzzer using cargo
    let fuzzer_bin = Command::new("cargo")
        .args(&["afl", "fuzz"])
        .arg("-i")
        .arg(&input_arg)
        .arg("-o")
        .arg(&corpus_dir)
        .args(&["--", &format!("../target/debug/{}", target)])
        .current_dir(&dir)
        .spawn()
        .context(format!("error starting {:?} to run {}", fuzzer, target))?
        .wait()
        .context(format!(
            "error while waiting for {:?} running {}",
            fuzzer, target
        ))?;

    if !fuzzer_bin.success() {
        Err(FuzzerQuit)?;
    }
    Ok(())
}

fn build_libfuzzer() -> Result<(), Error> {
    Ok(())
}

fn write_libfuzzer_target(fuzzer: Fuzzer, target: &str) -> Result<(), Error> {
    use std::io::Write;

    let fuzz_dir = fuzzer.dir()?.join("fuzz");
    println!("{:?}", fuzz_dir);

    let template_path = fuzzer.dir()?.join("template.rs");
    let template = fs::read_to_string(&template_path).context(format!(
        "error reading template file {}",
        template_path.display()
    ))?;

    // use `cargo fuzz add` to add new bin inside Cargo.toml
    // and create fuzz_targets dir
    // and create target.rs
    let _ = Command::new("cargo")
        .args(&["fuzz", "add", &target])
        .current_dir(&fuzz_dir)
        .spawn()
        .context(format!("error starting {:?} to run {}", fuzzer, target))?
        .wait()
        .context(format!(
            "error while waiting for {:?} running {}",
            fuzzer, target
        ));

    let target_dir = fuzz_dir.join("fuzz_targets");

    let path = target_dir.join(&format!("{}.rs", target));

    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path)
        .context(format!(
            "write_libfuzzer_target error writing fuzz target binary {}",
            path.display()
        ))?;

    let source = template.replace("###TARGET###", &target);
    file.write_all(source.as_bytes())?;
    Ok(())
}

fn run_libfuzzer(target: &str, timeout: Option<i32>) -> Result<(), Error> {
    let fuzzer = Fuzzer::Libfuzzer;
    write_libfuzzer_target(fuzzer, target)?;

    let fuzz_dir = fuzzer.dir()?.join("fuzz");

    let max_time = if let Some(timeout) = timeout {
        format!("-max_total_time={}", timeout)
    } else {
        "".into()
    };

    // TODO - fix maxtime
    println!("{:?}", max_time);

    let fuzzer_bin = Command::new("cargo")
        .args(&["-v", "fuzz", "run", &target])
        .current_dir(&fuzz_dir)
        .spawn()
        .context(format!("error starting {:?} to run {}", fuzzer, target))?
        .wait()
        .context(format!(
            "error while waiting for {:?} running {}",
            fuzzer, target
        ))?;

    if !fuzzer_bin.success() {
        Err(FuzzerQuit)?;
    }
    Ok(())
}

/// Write the fuzzing target
///
/// Copy the fuzzer/template.rs
/// Replace ###TARGET### by the target
fn write_fuzzer_target(fuzzer: Fuzzer, target: &str) -> Result<(), Error> {
    use std::io::Write;

    let template_path = fuzzer.dir()?.join("template.rs");
    let template = fs::read_to_string(&template_path).context(format!(
        "error reading template file {}",
        template_path.display()
    ))?;

    let target_dir = fuzzer.dir()?.join("src").join("bin");
    fs::create_dir_all(&target_dir).context(format!(
        "error creating fuzz target dir {}",
        target_dir.display()
    ))?;
    let path = target_dir.join(&format!("{}.rs", target));

    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path)
        .context(format!(
            "error writing fuzz target binary {}",
            path.display()
        ))?;

    let source = template.replace("###TARGET###", &target);
    file.write_all(source.as_bytes())?;
    println!("[WARF] {}: {} created", fuzzer, target);
    Ok(())
}

fn run_debug(target: &str) -> Result<(), Error> {
    //let fuzzer = Fuzzer::Honggfuzz;
    let cwd = env::current_dir().context("error getting current directory")?;
    let dir = cwd.join("debug");

    write_debug_target(dir.clone(), target)?;

    let debug_bin = Command::new("cargo")
        .args(&["build", "--bin", &format!("debug_{}", target)])
        .current_dir(&dir)
        .spawn()
        .context(format!("error starting {}", target))?
        .wait()
        .context(format!("error while waiting for {}", target))?;

    if !debug_bin.success() {
        Err(FuzzerQuit)?;
    }
    println!("[WARF] Debug: {} compiled", &format!("debug_{}", target));
    Ok(())
}

fn write_debug_target(debug_dir: PathBuf, target: &str) -> Result<(), Error> {
    use std::io::Write;

    let template_path = debug_dir.join("debug_template.rs");
    let template = fs::read_to_string(&template_path).context(format!(
        "error reading debug template file {}",
        template_path.display()
    ))?;

    let target_dir = debug_dir.join("src").join("bin");
    fs::create_dir_all(&target_dir).context(format!(
        "error creating debug target dir {}",
        target_dir.display()
    ))?;
    let path = target_dir.join(&format!("debug_{}.rs", target));

    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path)
        .context(format!(
            "error writing debug target binary {}",
            path.display()
        ))?;

    let source = template.replace("###TARGET###", &target);
    file.write_all(source.as_bytes())?;
    Ok(())
}

arg_enum! {
    #[derive(StructOpt, Debug, Clone, Copy, PartialEq, Eq)]
    enum Fuzzer {
        Afl,
        Honggfuzz,
        Libfuzzer
    }
}

impl Fuzzer {
    fn dir(&self) -> Result<PathBuf, Error> {
        let cwd = env::current_dir().context("error getting current directory")?;

        use Fuzzer::*;
        let p = match self {
            Afl => cwd.join("fuzzer-afl"),
            Honggfuzz => cwd.join("fuzzer-honggfuzz"),
            Libfuzzer => cwd.join("fuzzer-libfuzzer"),
        };

        Ok(p)
    }

    fn work_dir(&self) -> Result<PathBuf, Error> {
        let cwd = env::current_dir().context("error getting current directory")?;
        let cwd = cwd.join("workspace");

        use Fuzzer::*;
        let p = match self {
            Afl => cwd.join("afl"),
            Honggfuzz => cwd.join("hfuzz"),
            Libfuzzer => cwd.join("libfuzzer"),
        };

        Ok(p)
    }
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
