use std::fs;
use std::io::prelude::*;
use std::env;
use std::process;

pub mod tokens;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Error, excpected at least 1 arguments, got none");
        process::exit(0x1);
    }
    let file = read_file(&args[1]);
    println!("Source file: {}", file);
}

fn read_file(path: &String) -> String{
    // Open the file
    let file = fs::OpenOptions::new().read(true).open(path);

    // Check for any errors
    if !file.is_ok() {
        // Report the error
        eprintln!("Error opening file {}, error: {:?}", path, file.unwrap_err().kind());
        process::exit(0x1);
    }

    let mut file = file.unwrap();
    let mut source = String::new();
    match file.read_to_string(&mut source) {
        Err(a) => {eprintln!("Error reading file {:?}", a.kind()); process::exit(0x1);},
        _ => return source
    }
}
