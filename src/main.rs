#[macro_use] extern crate dyn_clone;
extern crate ansi_term;
extern crate num_cpus;

use std::time::Instant;
use std::{env, process};

pub mod token;
pub mod syntax_tree;
pub mod tree;
pub mod source_finder;

fn main() {
    let start_time = Instant::now();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Error, expected at least 1 arguments, got none");
        process::exit(0x1);
    }
    let initial_file_path = args[1].clone();
    let cpu_thread_count = num_cpus::get();
    let mut all_parsed_tokens: Vec<Vec<token::Line>> = vec!();
    source_finder::main::recursively_find_imports(&mut all_parsed_tokens,&cpu_thread_count,&initial_file_path);
    let elapsed = start_time.elapsed();
    println!("Tokens: {:?}",all_parsed_tokens);
    println!("\nTime taken: {:.5?}", elapsed);
}
