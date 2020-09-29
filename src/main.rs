#[macro_use] extern crate dyn_clone;
extern crate ansi_term;
extern crate num_cpus;

use std::time::Instant;
use crate::config::load_projects;
use crate::tree::find_events;

pub mod token;
pub mod tree;
pub mod import_manager;
pub mod error_handler;
pub mod config;

fn main() {
    let projects = load_projects();
    let start_time = Instant::now();
    let initial_file_path = projects.clone().root;
    let cpu_thread_count = num_cpus::get();
    let mut all_parsed_tokens: Vec<import_manager::ParsedFile> = vec!();
    import_manager::main::recursively_find_imports(&mut all_parsed_tokens,&cpu_thread_count,&initial_file_path,0);
    find_events(&all_parsed_tokens);
    let elapsed = start_time.elapsed();
    println!("Files imported: {:?}",all_parsed_tokens.len());
    println!("\nTime taken: {:.5?}", elapsed);
}
