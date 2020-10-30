extern crate ansi_term;
#[macro_use]
extern crate dyn_clone;
extern crate num_cpus;

use std::time::Instant;

use crate::config::load_projects;

pub mod token;
pub mod import_manager;
pub mod error_handler;
pub mod config;
mod parser;

fn main() {
    let projects = load_projects();
    let start_time = Instant::now();
    let all_parsed_tokens = import_manager::main::load_directory();
    let lexed = parser::lex(&all_parsed_tokens);
    println!("{:?}", lexed);
    let elapsed = start_time.elapsed();
    println!("Files imported: {}", all_parsed_tokens.len());
    println!("\nTime taken: {:.5?}", elapsed);
}
