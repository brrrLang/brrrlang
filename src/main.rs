#[macro_use] extern crate dyn_clone;
extern crate ansi_term;
extern crate num_cpus;

use std::time::Instant;
use crate::config::load_projects;

pub mod token;
pub mod import_manager;
pub mod error_handler;
pub mod config;

fn main() {
    let projects = load_projects();
    let start_time = Instant::now();
    let initial_file_path = projects.clone().root;
    let cpu_thread_count = num_cpus::get();
    let mut all_parsed_tokens: Vec<import_manager::ParsedFile> = vec!();
    import_manager::main::load_directory(&mut all_parsed_tokens);
    let elapsed = start_time.elapsed();
    println!("Files imported: {:?}",all_parsed_tokens.len());
    println!("\nTime taken: {:.5?}", elapsed);
}
