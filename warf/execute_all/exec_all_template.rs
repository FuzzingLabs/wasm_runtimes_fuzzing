#![allow(unused)]
extern crate fuzz_targets;
use fuzz_targets::*;

use std::env;
use std::fs::File;
use std::io;
use std::io::Read;

use colored::*;

use std::time::Instant;

/// Read the contents from file path
fn read_contents_from_path(path_str: &String) -> Result<Vec<u8>, io::Error> {
    let mut buffer: Vec<u8> = Vec::new();
    let file_path = std::path::PathBuf::from(path_str);

    println!("file_to_process: {:?}\n", file_path);

    let mut file = File::open(file_path)?;
    file.read_to_end(&mut buffer)?;
    drop(file);
    Ok(buffer)
}

fn log_benchmark(it: Instant) {
    let elapsed = (Instant::now() - it).as_secs_f64();
    println!("{}", format!("benchmark (sec): {:5.10}", elapsed / 1024.).yellow());
}

fn is_ok(target: String, res: bool) {
    if res {
        println!("{}", format!("[O] {}: Ok()", target).green());
    } else {
        println!("{}", format!("[X] {}: Err()", target).red());
    }
}

fn main() {
    println!("Execution of all runtime engine");
    let args: Vec<String> = env::args().collect();

    // verify file_to_process is provided
    if args.len() != 2 {
        println!("Usage: {} <wasm_to_process>\n", &args[0]);
        return;
    }

    // read data from provided file
    let data = read_contents_from_path(&args[1]).expect("cannot read file content");

    // call all fuzzing targets
###TARGETS###

    println!("\nNo crash, everything is OK\n");
}
