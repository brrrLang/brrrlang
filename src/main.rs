#[macro_use] extern crate dyn_clone;
extern crate ansi_term;

use std::time::Instant;
use std::{env, process};

pub mod token;
pub mod syntax_tree;
pub mod tree;

fn main() {
    let start_time = Instant::now();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Error, expected at least 1 arguments, got none");
        process::exit(0x1);
    }
    let file = token::tokenizer::read_file(&args[1]);
    let _tokens = token::tokenizer::tokenize(&file);
    let elapsed = start_time.elapsed();
    println!("\nTime taken: {:.5?}", elapsed);
}
