use std::{fs, io};
use std::io::prelude::*;
use std::process::exit;

use clap::{App, Arg};

mod token;
mod error_handler;

fn main() {
    let matches = App::new("brrrLang Compiler")
        .version("0.1.0")
        .about("All the tools you need for brrrLang")
        .arg(Arg::with_name("compile")
            .short("c")
            .long("compile")
            .value_name("FILE")
            .help("The file to be compiled")
            .takes_value(true)
            .required(true)
        ).get_matches();


    let root = matches.value_of("compile").unwrap_or_else(|| {
        exit(1);
    }).to_string();
    let file = read_file(&root).unwrap();

    let parsed_file = token::tokenizer::ParsedFile::new(&file, &root);

    println!("{:#?}", parsed_file);
}

fn read_file(path: &str) -> Result<String, io::Error> {
    let mut file = fs::File::open(path)?;
    let mut result = String::new();
    file.read_to_string(&mut result)?;
    Ok(result)
}
