extern crate ansi_term;

use std::fs;
use std::io::prelude::*;
use std::env;
use std::process;
use std::sync::{Arc,Mutex};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use ansi_term::Colour;

pub mod tokens;

fn main() {
    let start_time = Instant::now();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Error, excpected at least 1 arguments, got none");
        process::exit(0x1);
    }
    let file = read_file(&args[1]);
    let tokens = tokenize(&file);
    let elapsed = start_time.elapsed();
    println!("\nTime taken: {:.5?}", elapsed);
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

fn compile_error(error: ErrorWarning) {
    if error.line_num == -1 {
        println!("{} {}",Colour::Red.bold().paint(format!("\nCompilation error in process {}:", error.area_errored )),Colour::Red.paint(format!("{}",error.message)));
    } else {
        println!("{} {}",Colour::Red.bold().paint(format!("\nCompilation error in process {}, line {}:", error.area_errored, error.line_num )),Colour::Red.paint(format!("{}",error.message)));
    }
    process::exit(0);
    
}

fn tokenize(source: &String) -> Vec<tokens::Token> {
    let mut tokens: Vec<tokens::Token> = vec!();
    let mut lines: Arc<Mutex<Vec<tokens::Line>>> = Arc::new(Mutex::new(vec!()));
    let chars: Vec<char> = source.chars().map( |x| match x { //Converts to Vec<char> and removes tabs and somtimes occouring \r newline
        '\t' => ' ',
        '\r' => ' ',
        _ => x
    }).collect::<Vec<char>>();
    let mut current_char: char = ' ';
    let mut last_char: char;
    let mut source_loc = 0;
    let mut line: tokens::Line;
    let mut line_num: usize = 0;
    let mut actual_line_num: usize = 1;
    let mut line_start: usize = 0;
    let mut line_end: usize = 0;
    let mut scope_indentation = 0;
    let mut scope_id = 0;
    let mut scope_id_chain = vec!();
    let mut tokenize_thread_handles: Vec<thread::JoinHandle<()>> = vec!();

    let max_threads = 24;
    let mut num_threads = 0;
    let (channel_tx, channel_rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

	while source_loc < chars.len() {
        last_char = current_char.clone();
		current_char = chars[source_loc];
        if current_char == ';'  || current_char == '{' || current_char == '}' {
            line = tokens::Line::new();
            line.line_text = String::from(chars[line_start..line_end+1].iter().collect::<String>().trim());
            if current_char == '{' {
                scope_indentation+=1;
                scope_id+=1;
                scope_id_chain.push(scope_id);
            } else if current_char == '}'{
                scope_indentation-=1;
                scope_id_chain.pop();
            }
            line.line_num = line_num;
            line.actual_line_num = actual_line_num;
            line.scope_indentation = scope_indentation;
            line.scope_id_chain = scope_id_chain.clone();
            line.line_char_start = line_start;
            line.line_char_end = line_end;
            println!("Line {:?}",line.line_text);
            //Theard handler
            if num_threads < 24 {
                tokenize_thread_handles.push(tokenizer_thread(&line, &lines, &channel_tx));
                num_threads+=1;
            } else {
                channel_rx.recv().unwrap();
                tokenize_thread_handles.push(tokenizer_thread(&line, &lines, &channel_tx));
            }
            // thread::sleep(Duration::from_millis(500));
            line_start = source_loc + 1;
            line_end = line_start;
            line_num +=1;
        } else if current_char == '/' && last_char == '/' { //One line comment
            println!("Advoiding comment at char {}", source_loc);
            while source_loc < chars.len() && current_char != '\n' {
                current_char = chars[source_loc];
                source_loc+=1;
            }
            line_start = source_loc + 1;
            actual_line_num += 1;
            line_end = line_start;
        } else if current_char == '/' && last_char == '*' { //Multiline comment
            println!("\n\n\n\nAdvoiding a multiline comment");
            while source_loc < chars.len() && !(last_char == '*' && current_char == '/') {
                last_char = current_char.clone();
                current_char = chars[source_loc];
                if current_char == '\n' {
                    actual_line_num += 1;
                }
                source_loc+=1;
            }
            line_start = source_loc + 1;
            line_end = line_start;
        } else if current_char == '"'{
            source_loc += 1;
            last_char = current_char.clone();
            current_char = chars[source_loc];
            while current_char != '"' || last_char == '\\' {
                if source_loc >= chars.len() {
                    let error = ErrorWarning::new(String::from("tokenize"),actual_line_num as i32,String::from("String was not closed"));
                    compile_error(error);
                }
                last_char = current_char.clone();
                current_char = chars[source_loc];
                source_loc += 1;
                line_end+=1;
            }
            source_loc -= 1;
            line_end+=1;
        } else if current_char == '\''{
            source_loc += 1;
            last_char = current_char.clone();
            current_char = chars[source_loc];
            while current_char != '\'' || last_char == '\\' {
                if source_loc >= chars.len() {
                    let error = ErrorWarning::new(String::from("tokenize"),actual_line_num as i32,String::from("String was not closed"));
                    compile_error(error)
                }
                last_char = current_char.clone();
                current_char = chars[source_loc];
                source_loc += 1;
                line_end+=1;
            }
            source_loc -= 1;
            line_end+=1;
        } else if current_char == '\n' {
            actual_line_num += 1;
            line_end += 1;
        } 
        else {
            line_end+=1;
        }


		source_loc += 1;
    }
    for handle in tokenize_thread_handles {
        handle.join().unwrap();
    }
    if scope_indentation != 0 {
        compile_error(ErrorWarning::new(String::from("tokenize"), -1, String::from("Missing closing clurly brackets")));
    }

    println!("Lines = {:#?}",lines);
	return tokens;
}
fn tokenizer_thread(line: &tokens::Line, lines_data: &Arc<Mutex<Vec<tokens::Line>>>, channel_tx: &mpsc::Sender<i32>) -> thread::JoinHandle<()> {
    println!("\n\n\n\n\nTokonizer thread started");
    let lines_data = Arc::clone(lines_data);
    let mut line_local = line.clone();
    let channel_thread_tx = channel_tx.clone();
    let handle = thread::spawn(move || {
        let mut tokens: Vec<tokens::Token> = vec!();
        println!("Line text: {}", line_local.line_text);
        let line_text: Vec<char> = line_local.line_text.clone().chars().collect();
        
        /*
        Splits the line into the relavent strings by operators and spaces
        */
        let mut line_split: Vec<String> = vec!(String::new());
        let mut line_split_pointer = 0;
        let mut i: usize = 0;
        println!("Line length: {}",line_text.len());
        while i < line_text.len() {
            println!("Processing char: {} num: {}",line_text[i],i);
            if line_text[i] == ' ' || line_text[i] == '\n'{
                if line_split[line_split_pointer].len() != 0 {
                    line_split.push(String::new());
                    line_split_pointer+=1;
                }
            } else if (line_text[i] == '=' && line_text[i+1] != '>' && line_text[i+1] != '=' && line_text[i+1] != '<') || line_text[i] == '+' || line_text[i] == '-' || line_text[i] == '*' || line_text[i] == '(' || line_text[i] == ')' || line_text[i] == '[' || line_text[i] == ']' || line_text[i] == '{' || line_text[i] == '}' || line_text[i] == ',' || line_text[i] == '<' || line_text[i] == '>'  {
                if line_split[line_split_pointer].len() != 0 {
                    line_split_pointer+=2;
                    line_split.push(String::new());
                    line_split.push(String::new());
                    line_split[line_split_pointer-1].push(line_text[i]);
                } else {
                    line_split_pointer+=2;
                    line_split.push(String::new());
                    line_split.push(String::new());
                    line_split[line_split_pointer-1].push(line_text[i]);
                }
            } else if (line_text[i] == '/' && line_text[i+1] == '/') || (line_text[i] == ':' && line_text[i+1] == ':'){
                line_split_pointer+=2;
                line_split.push(String::new());
                line_split.push(String::new());
                line_split[line_split_pointer-1].push(line_text[i]);
                line_split[line_split_pointer-1].push(line_text[i+1]);
                i+=1;
            } else if line_text[i] == ':' && line_text[i+1] != ':'{
                line_split_pointer+=2;
                line_split.push(String::new());
                line_split[line_split_pointer-1].push(line_text[i]);
            } else if line_text[i] == ';' { //ignore
            } else if line_text[i] == '=' && (line_text[i+1] == '=' || line_text[i+1] == '>' || line_text[i+1] == '<') {
                line_split_pointer+=3;
                line_split.push(String::new());
                line_split.push(String::new());
                line_split[line_split_pointer-1].push(line_text[i]);
                line_split[line_split_pointer-1].push(line_text[i+1]);
                i+=1;
            } else if line_text[i] == '!' && line_text[i+1] == '=' {
                line_split_pointer+=3;
                line_split.push(String::new());
                line_split.push(String::new());
                line_split[line_split_pointer-1].push(line_text[i]);
                line_split[line_split_pointer-1].push(line_text[i+1]);
                i+=1;
            }
            else if line_text[i] == '@' && i == 0 {
                line_split[line_split_pointer].push(line_text[i]);
                line_split_pointer +=1
            } else if line_text[i] == '"' { // don't want to split strings
                i+=1;
                while line_text[i] != '"' && line_text[i-1] != '\\' {
                    println!("String char: {} num: {}",line_text[i],i);
                    line_split[line_split_pointer].push(line_text[i]);
                    i+=1;
                }
                line_split_pointer+=1;
            } else if line_text[i] == '\'' { //and you can't have amberguity between string decluration symbols
                line_split[line_split_pointer].push(line_text[i]);
                i+=1;
                while line_text[i] != '\'' && line_text[i-1] != '\\' {
                    println!("String char: {} num: {}",line_text[i],i);
                    line_split[line_split_pointer].push(line_text[i]);
                    i+=1;
                }
                line_split[line_split_pointer].push(line_text[i]);
                line_split_pointer+=1;
            }
            else {
                line_split[line_split_pointer].push(line_text[i]);
            }
            
            while line_split_pointer >= line_split.len(){
                line_split.push(String::new());
            }
            i+=1;
        }
        line_split_pointer = 0;
        while line_split_pointer < line_split.len() { //Clear out empty splits
            if line_split[line_split_pointer] == String::new() {
                println!("Removing {}",line_split[line_split_pointer]);
                line_split.remove(line_split_pointer);
                // x+=1;
            } else {
                line_split_pointer+=1;
            }
        }
        println!("Lines split from: {:?} to: {:?}",line_local.line_text,line_split);
        line_local.line_split = line_split;
        /*
        Matches the keywords using context
        */
        i = 0;
        //Everything should be done by now
        line_local.line_token = tokens;
        let mut mutex_lines_data = lines_data.lock().unwrap();
        while mutex_lines_data.len() <= line_local.line_num { mutex_lines_data.push(tokens::Line::new())}
        mutex_lines_data[line_local.line_num] = line_local.clone();
        channel_thread_tx.send(0).unwrap();
    });
    return handle;
} 

pub struct ErrorWarning {
    area_errored: String,
    line_num: i32,
    message: String
}
impl ErrorWarning {
    pub fn new(area_errored: String, line_num: i32, message: String) -> ErrorWarning {
        return ErrorWarning {
            area_errored: area_errored,
            line_num: line_num,
            message: message
        }
    }
}