use crate::token;
use std::process;

use crate::import_manager::*;
use crate::error_handler;

pub fn recursively_find_imports(all_parsed_files: &mut Vec<ParsedFile>,cpu_thread_count: &usize, file_path: &String) {
    println!("Parsing File: {}",file_path);
    let _tokens = token::tokenizer::parse_file(&file_path,&cpu_thread_count);
    all_parsed_files.push(ParsedFile{
        lines: _tokens.clone(),
        file_path: file_path.clone(),
    });
    for a in _tokens.iter() {
        if a.line_token.len() >= 3 {
            match a.line_token[1] {
                token::Token::Import =>  {
                    let file_path_to_import = match &a.line_token[2] {
                        token::Token::Identifier(s) => s,
                        _ => {println!("Error in import tokenization"); process::exit(0x0100)},
                    };
                    if !all_parsed_files.iter().any(|i| i.file_path == *file_path_to_import) {
                        recursively_find_imports(all_parsed_files,cpu_thread_count, file_path_to_import);
                    }
                },
                token::Token::Use => {
                    error_handler::warning::warning_reporter(error_handler::Warning::new(
                        String::from("Importing"),
                        a.actual_line_num as i32,
                        String::from("Hah lol, you expect to be able to use a library? Get recked noob."),
                        a.line_text.clone(),
                    ))
                }
                _ => {}
            }
        }
    }
}