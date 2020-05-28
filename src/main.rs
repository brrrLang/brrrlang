use std::fs;
use std::io::prelude::*;
use std::env;
use std::process;

pub mod tokens;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Error, excpected at least 1 arguments, got none");
        process::exit(0x1);
    }
    let file = read_file(&args[1]);
    println!("Source file: {}", file);
    let tokens = tokenize(&file);
    println!("Lines = ")
}

fn read_file(path: &String) -> String{
    // Open the file
    let file = fs::OpenOptions::new().read(true).open(path);

    // Check for any errors
    if !file.is_ok() {
        // Report the error
        eprintln!("Error opening file {}, error: {:?}", path, file.unwrap_err().kind());
        process::exit(0x1);
    }

    let mut file = file.unwrap();
    let mut source = String::new();
    match file.read_to_string(&mut source) {
        Err(a) => {eprintln!("Error reading file {:?}", a.kind()); process::exit(0x1);},
        _ => return source
    }
}
fn tokenize(source: &String) -> Vec<tokens::Token> {
    let mut tokens: Vec<tokens::Token> = vec!();
    let mut lines: Vec<tokens::Line> = vec!();
    let chars: Vec<char> = source.chars().map( |x| match x { //Converts to Vec<char> and removes tabs and somtimes occouring \r newline
        '\t' => ' ',
        '\r' => ' ',
        _ => x
    }).collect::<Vec<char>>();
    let mut current_char: char = ' ';
    let mut last_char: char;
    let mut source_loc = 0;
    let mut line: tokens::Line;
    let mut line_start: usize = 0;
    let mut line_end: usize = 0;
    let mut scope_indentation = 0;
    let mut scope_id = 0;

	while source_loc < chars.len() {
        last_char = current_char.clone();
		current_char = chars[source_loc];
        if current_char == ';'  || current_char == '{'{
            line = tokens::Line::new();
            line.line_text = String::from(chars[line_start..line_end].iter().collect::<String>().trim());
            line.scope_indentation = scope_indentation;
            line.scope_id = scope_id;
            line.line_char_start = line_start;
            line.line_char_end = line_end;
            println!("Line {:?}",line);
            line_start = source_loc + 1;
            line_end = line_start;
        }
        if current_char == '/' && last_char == '/' { //One line comment
            println!("Advoiding comment");
            while source_loc < chars.len() && current_char != '\n' {
                current_char = chars[source_loc];
                println!("Current Char: {:?}",current_char);
                source_loc+=1;
            }
            line_start = source_loc + 1;
            line_end = line_start;
        }
        else {
            line_end+=1;
        }


		source_loc += 1;
    }
    println!("Lines = {:?}",lines);
	return tokens;
}
