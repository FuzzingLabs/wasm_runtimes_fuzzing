extern crate fuzz_targets;
use fuzz_targets::fuzz_###TARGET### as fuzz_target;

use std::env;
use std::fs::File;
use std::io;
use std::io::Read;

/// Read the contents from file path
fn read_contents_from_path(path_str: &String) -> Result<Vec<u8>, io::Error> {
    let mut buffer: Vec<u8> = Vec::new();
    let file_path = std::path::PathBuf::from(path_str);

    println!("file_to_process: {:?}", file_path);

    let mut file = File::open(file_path)?;
    file.read_to_end(&mut buffer)?;
    drop(file);
    Ok(buffer)
}

fn main() {
    println!("Start debugging of ###TARGET###");
    let args: Vec<String> = env::args().collect();

    // verify file_to_process is provided
    if args.len() != 2 {
        println!("Usage: ###TARGET### <file_to_process>\n");
        return;
    }

    // read data from provided file
    let data = read_contents_from_path(&args[1]).expect("cannot read file content");

    // call the fuzzing target
    fuzz_target(&data);

    println!("No crash, everything is OK\n");
}
