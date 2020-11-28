use std::{fs, io};
use std::io::prelude::*;

use clap::{App, SubCommand};
use ansi_term::Color;
use std::path::PathBuf;
use std::panic::catch_unwind;

mod token;
mod config;
mod error_handler;

fn main() {
    let matches = App::new("brrrLang Compiler")
        .version("0.1.0")
        .about("All the tools you need for brrrLang")
        .subcommand(SubCommand::with_name("run")
            .about("Compiles and runs you project")
            .version("0.1.0"))
        .get_matches();

    // Run subcommand
    if let Some(_matches) = matches.subcommand_matches("run") {
        compile();
    }
}

fn compile() {
    // Load the project configuration
    let project = config::load_projects();

    // Terminal formatting stuff
    const GREEN: Color = Color::Green;
    const WHITE: Color = Color::White;

    println!("{} {}", GREEN.bold().paint("Building"),
             WHITE.italic().paint(&project.project_name));

    // Get all of the files in the src directory
    let paths = fs::read_dir("./src").unwrap();

    //  Create a Senders and Receivers for multithreading goodness.
    let (tx, rx) = std::sync::mpsc::channel();
    for path in paths {
        let sender = tx.clone();
        std::thread::spawn(move || {
            let path = path.unwrap().path();
            let path_str = path.file_name().unwrap().to_str().unwrap().to_string();
            let source = read_file(&path).unwrap();
            match catch_unwind(|| token::tokenizer::ParsedFile::new(&source, &path_str)) {
                Ok(parsed) => sender.send(parsed).unwrap(),
                Err(_) => {}
            }

        });
    }

    drop(tx);

    // Vector to store parsed files
    let mut parsed_files = vec!();

    // Collect all of the parsed files
    for result in rx {
        parsed_files.push(result);
    }

    for file in parsed_files.iter() {
        println!("File: {}\nSource:{}\nTokens{:#?}\n", file.path,file.source, file.tokens)
    }

    println!("{} {}", GREEN.bold().paint("Built"), WHITE.italic().paint(&project.project_name));
}

fn read_file(path: &PathBuf) -> Result<String, io::Error> {
    let mut file = fs::File::open(path)?;
    let mut result = String::new();
    file.read_to_string(&mut result)?;
    Ok(result)
}
