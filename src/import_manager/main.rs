use crate::token;
use std::{process, fs};
use std::fs::File;
use std::io::prelude::*;
use rayon::prelude::*;

use crate::import_manager::*;
use crate::error_handler;
use dyn_clone::private::fs::DirEntry;

pub fn load_directory() -> Vec<ParsedFile> {
    let ye =
        // Get all files in the src directory
        fs::read_dir("./src")
            .unwrap()
            // Unwrap all paths
            .map(|a| a.unwrap())
            // Keep only files ending in .bf
            .filter(|path| path.file_name().to_str().unwrap().to_string().ends_with(".bl"))
            // Convert to a parallel iterator for Z O O M
            .collect::<Vec<DirEntry>>()
            .par_iter()
            // Read and parse all files
            .map(|path| {
                println!("Parsing {:?}", path);
                let mut data = String::new();
                // Open the file
                return match File::open(path.path()).unwrap().read_to_string(&mut data) {
                    // Check there were no errors when the file was red
                    Err(e) => {
                        eprintln!("Error reading file {} \n", e);
                        None
                    }
                    // Parse the opened file
                    _ => {
                        let tokens = token::tokenizer::tokenize(&data, &num_cpus::get(), &0);
                        println!("{:?}", tokens);
                        // Add the files to a list of files
                        Some(ParsedFile {
                            lines: tokens.clone(),
                            file_path: path.file_name().to_str().unwrap().to_string(),
                        })
                    }
                };
            })
            // Remove any None's
            .filter(|a| a.is_some())
            .collect::<Vec<Option<ParsedFile>>>();

    // Unwrap all files
    let mut d = vec!();
    for a in ye {
        d.push(a.unwrap());
    }

    d
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