use crate::token;
use std::{process, fs};
use std::fs::File;
use std::io::prelude::*;
use rayon::prelude::*;

use crate::import_manager::*;
use crate::error_handler;
use dyn_clone::private::fs::DirEntry;

pub fn load_directory(all_parsed_files: &mut Vec<ParsedFile>) -> Vec<ParsedFile> {
    fs::read_dir("./")
        .unwrap()
        .map(|a| a.unwrap())
        .collect::<Vec<DirEntry>>()
        .par_iter()
        .map(|path: std::fs::DirEntry| {
            if path.file_name().to_str().unwrap().to_string().ends_with(".bl") {
                let mut data = String::new();
                match File::open(path.path()).unwrap().read_to_string(&mut data) {
                    Err(e) => eprintln!("Error reading file {} \n", e),
                    _ => {
                        ParsedFile {
                            lines: token::tokenizer::tokenize(&data, &num_cpus::get(), &0),
                            file_path: path.file_name().to_str().unwrap().to_string(),
                        }
                    }
                };
            }
        }).collect::<Vec<ParsedFile>>()
}

pub fn recursively_find_imports(all_parsed_files: &mut Vec<ParsedFile>, cpu_thread_count: &usize, file_path: &String, scope_id_start_pos: i32) {
    println!("Parsing File: {}", file_path);
    let file_tokens = token::tokenizer::parse_file(&file_path, &cpu_thread_count, &scope_id_start_pos);
    all_parsed_files.push(ParsedFile {
        lines: file_tokens.clone(),
        file_path: file_path.clone(),
    });
    // for token_line in file_tokens.iter() {
    //     if token_line.line_token.len() >= 3 {
    //         match token_line.line_token[1] {
    //             token::Token::Import =>  { //Internal Dependence
    //                 let file_path_to_import = match &token_line.line_token[2] {
    //                     token::Token::Identifier(s) => s,
    //                     _ => {println!("Error in import tokenization"); process::exit(0x0100)},
    //                 };
    //                 if !all_parsed_files.iter().any(|i| i.file_path == *file_path_to_import) {
    //                     recursively_find_imports(all_parsed_files,cpu_thread_count, file_path_to_import,scope_id_start_pos);
    //                 }
    //             },
    //             token::Token::Use => { //External Dependence
    //                 error_handler::warning::warning_reporter(error_handler::Warning::new(
    //                     String::from("Importing"),
    //                     token_line.actual_line_num as i32,
    //                     String::from("Project conf needs to be implemented before external dependencies"),
    //                     token_line.line_text.clone(),
    //                 ))
    //             }
    //             _ => {}
    //         }
    //     }
    // }
}