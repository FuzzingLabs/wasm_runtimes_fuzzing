extern crate clap;
extern crate failure;
extern crate fs_extra;
extern crate regex;
extern crate structopt;

use crate::fuzzers::FuzzerQuit;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use failure::{Error, ResultExt};

use std::io::Write;

use crate::env::{root_dir, workspace_dir};
use crate::targets::{get_targets, prepare_targets_workspace};

fn prepare_exec_all_workspace(out_dir: &str) -> Result<(), Error> {
    let debug_init_dir = root_dir()?.join("execute_all");
    let dir = root_dir()?.join("workspace");

    let debug_dir = dir.join(out_dir);
    fs::create_dir_all(&debug_dir)
        .context(format!("unable to create {} dir", debug_dir.display()))?;

    let src_dir = debug_dir.join("src");
    fs::create_dir_all(&src_dir).context(format!("unable to create {} dir", src_dir.display()))?;

    fs::copy(
        debug_init_dir.join("Cargo.toml"),
        debug_dir.join("Cargo.toml"),
    )?;
    fs::copy(
        debug_init_dir.join("src").join("lib.rs"),
        src_dir.join("lib.rs"),
    )?;
    Ok(())
}

pub fn run_exec_all(wasm: String, benchmark: bool) -> Result<(), Error> {
    let debug_dir = root_dir()?.join("workspace").join("execute_all");

    prepare_targets_workspace()?;
    prepare_exec_all_workspace("execute_all")?;

    write_exec_all_target(&debug_dir, benchmark)?;

    let debug_bin = Command::new("cargo")
        .args(&["build", "--release", "--bin", "exec_all"])
        .args(&["--out-dir=..", "-Z", "unstable-options"]) // copy exec_all binary in workspace folder
        .current_dir(&debug_dir)
        .spawn()
        .context("execute_all")?
        .wait()
        .context("execute_all")?;

    if !debug_bin.success() {
        Err(FuzzerQuit)?;
    }
    println!(
        "[WARF] execute_all compiled here: {:#?}/exec_all",
        workspace_dir()?
    );

    // Execute the exec_all
    let debug_bin = Command::new(workspace_dir()?.join("exec_all"))
        .arg(wasm)
        .current_dir(root_dir()?)
        .spawn()
        .context("exec_all")?
        .wait()
        .context("exec_all")?;

    if !debug_bin.success() {
        Err(FuzzerQuit)?;
    }

    Ok(())
}

fn write_exec_all_target(debug_dir: &PathBuf, benchmark: bool) -> Result<(), Error> {
    // TODO - make it cleaner
    let template_path = root_dir()?.join("execute_all").join("exec_all_template.rs");
    let template = fs::read_to_string(&template_path).context(format!(
        "error reading execute_all template file {}",
        template_path.display()
    ))?;

    let target_dir = debug_dir.join("src").join("bin");
    fs::create_dir_all(&target_dir).context(format!(
        "error creating execute_all target dir {}",
        target_dir.display()
    ))?;
    let path = target_dir.join("exec_all.rs");

    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path)
        .context(format!(
            "error writing execute_all target binary {}",
            path.display()
        ))?;

    // TODO
    let mut targets = String::new();
    for target in get_targets() {
        if benchmark {
            targets.push_str("    let it = Instant::now();\n");
        }
        targets.push_str(&format!("    let res = debug_{}(&data);\n", target));
        targets.push_str(&format!("    is_ok(\"{}\".to_string(), res);\n\n", target));
        if benchmark {
            targets.push_str("    log_benchmark(it);\n");
        }
    }
    let source = template.replace("###TARGETS###", &targets);
    file.write_all(source.as_bytes())?;
    Ok(())
}
