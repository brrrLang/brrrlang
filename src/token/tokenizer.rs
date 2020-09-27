use std::fs;
use std::io::prelude::*;
use std::process;
use std::sync::{Arc,Mutex};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
#[allow(unused_imports)]
use std::time::{Duration, Instant};

use crate::token::*;
use crate::error_handler;

pub fn parse_file(file_name: &String,cpu_thread_count: &usize, scope_id_start_pos: &i32) -> Vec<Line> {
	let file = read_file(&file_name);
	let _tokens = tokenize(&file,&cpu_thread_count,scope_id_start_pos);
	return _tokens;
}

pub fn read_file(path: &String) -> String{
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
	// Reading the file
	match file.read_to_string(&mut source) {
		// Reporting the errors and exiting the compiler
		Err(a) => {eprintln!("Error reading file {:?}", a.kind()); process::exit(0x1);},
		// Returning the read in source code
		_ => return source
	}
}



pub fn tokenize(source: &String, cpu_thread_count: &usize, scope_id_start_pos: &i32) -> Vec<Line> {
	let lines: Arc<Mutex<Vec<Line>>> = Arc::new(Mutex::new(vec!()));
	let chars: Vec<char> = source.chars().map( |x| match x { //Converts to Vec<char> and removes tabs and sometimes occurring \r newline
		'\t' => ' ',
		'\r' => ' ',
		_ => x
	}).collect::<Vec<char>>();
	let mut current_char: char = ' ';
	let mut last_char: char;
	let mut source_loc = 0;
	let mut line: Line;
	let mut line_num: usize = 0;
	let mut actual_line_num: usize = 1;
	let mut line_start: usize = 0;
	let mut line_end: usize = 0;
	let mut scope_indentation = 0;
	let mut scope_id = scope_id_start_pos.clone();
	let mut scope_id_chain = vec!();
	let mut tokenize_thread_handles: Vec<thread::JoinHandle<()>> = vec!();
	let mut num_threads: usize = 0;
	let (channel_tx, channel_rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

	while source_loc < chars.len() {
		last_char = current_char.clone();
		current_char = chars[source_loc];
		if current_char == ';'  || current_char == '{' || current_char == '}' {
			line = Line::new();
			line.line_text = String::from(
				chars[
					line_start..{line_end+ if current_char != ';' {1} else {0}}
					]
					.iter().collect::<String>().trim()
			);
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
			//Thread handler
			if num_threads <= *cpu_thread_count {
				tokenize_thread_handles.push(tokenizer_thread(&line, &lines, &channel_tx, &num_threads));
				num_threads+=1;
			} else {
				channel_rx.recv().unwrap();
				tokenize_thread_handles.push(tokenizer_thread(&line, &lines, &channel_tx, &num_threads));
				num_threads+=1;
			}
			// thread::sleep(Duration::from_millis(500));
			line_start = source_loc + 1;
			line_end = line_start;
			line_num +=1;
		} else if current_char == '/' && last_char == '/' { //One line comment
			while source_loc < chars.len() && current_char != '\n' {
				current_char = chars[source_loc];
				source_loc+=1;
			}
			line_start = source_loc;
			actual_line_num += 1;
			line_end = line_start+1;
		} else if current_char == '/' && last_char == '*' { //Multiline comment;
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
					let error = error_handler::Error::new(String::from("tokenize"),actual_line_num as i32,String::from("String was not closed"),String::new());
					error_handler::error::error_reporter(error);
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
					let error = error_handler::Error::new(String::from("tokenize"),actual_line_num as i32,String::from("String was not closed"),String::new());
					error_handler::error::error_reporter(error)
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
		error_handler::error::error_reporter(
			error_handler::Error::new(String::from("tokenize"), -1, String::from("Missing closing curly brackets"),String::new())
		);
	}

	//println!("Lines = {:#?}",lines);
	let lines = Arc::clone(&lines);
	return lines.lock().unwrap().clone();
}
pub fn tokenizer_thread(line: &Line, lines_data: &Arc<Mutex<Vec<Line>>>, channel_tx: &mpsc::Sender<i32>, num_threads: &usize) -> thread::JoinHandle<()> {
	// println!("\n {:?} Tokenizer thread started", num_threads);
	let lines_data = Arc::clone(lines_data);
	let mut line_local = line.clone();
	let channel_thread_tx = channel_tx.clone();
	let thread_builder = thread::Builder::new()
							.name(num_threads.to_string().into());
	let handle = thread_builder.spawn(move || {
		// println!("Line text: {}", line_local.line_text);
		let line_text: Vec<char> = line_local.line_text.clone().chars().collect();

		/*
		Splits the line into the relevant strings by operators and spaces
		*/
		let mut line_split: Vec<String> = vec!(String::new());
		let mut line_split_pointer = 0;
		let mut i: usize = 0;
		while i < line_text.len() {
			if line_text[i] == ' ' || line_text[i] == '\n'{
				if line_split[line_split_pointer].len() != 0 {
					line_split.push(String::new());
					line_split_pointer+=1;
				}
			} 
			else if 
				(line_text[i] == '=' && line_text[i+1] != '>' && line_text[i+1] != '=' && line_text[i+1] != '<') || line_text[i] == '+' || line_text[i] == '-' || line_text[i] == '*' || 
				line_text[i] == '(' || line_text[i] == ')' || line_text[i] == '[' || line_text[i] == ']' || line_text[i] == '{' || line_text[i] == '}' || line_text[i] == ','|| line_text[i] == '.' || 
				(line_text[i] == '<' && line_text[i+1] != '=') || (line_text[i] == '>' && line_text[i+1] != '=') || line_text[i] == ';' || (line_text[i] == '!' && line_text[i+1] != '=')
			{
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
			} else if (line_text[i] == '=' || line_text[i] == '>' || line_text[i] == '<') && line_text[i+1] == '='  {
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
					line_split[line_split_pointer].push(line_text[i]);
					i+=1;
				}
				line_split_pointer+=1;
			} else if line_text[i] == '\'' { //and you can't have ambiguity between string declaration symbols
				line_split[line_split_pointer].push(line_text[i]);
				i+=1;
				while line_text[i] != '\'' && line_text[i-1] != '\\' {
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
				line_split.remove(line_split_pointer);
				// x+=1;
			} else {
				line_split_pointer+=1;
			}
		}
		line_local.line_split = line_split.clone();
		/*
		Matches the keywords using context
		*/
		i = 0;
		let mut string_token: String;
		let mut char_token: Vec<char>;
		let mut line_tokens: Vec<Token> = vec!();
		while i < line_split.len() {
			string_token = line_split[i].clone();
			// Check if keyword line_tokens.push()
			match string_token.as_str() {
				"@" => line_tokens.push(Token::Tag),
				"(" => line_tokens.push(Token::LBrace),
				")" => line_tokens.push(Token::RBrace),
				"{" => line_tokens.push(Token::LCurlyBrace),
				"}" => line_tokens.push(Token::RCurlyBrace),
				"[" => line_tokens.push(Token::LSquareBrace),
				"]" => line_tokens.push(Token::RSquareBrace),
				"enum" => line_tokens.push(Token::Enum),
				";" => {},
				"." => line_tokens.push(Token::Period),
				"," => line_tokens.push(Token::Comma),
				"while" => line_tokens.push(Token::Comma),
				"for" => line_tokens.push(Token::Comma),
				"until" => line_tokens.push(Token::Comma),
				"if" => line_tokens.push(Token::Comma),
				"else" => line_tokens.push(Token::Comma),
				"_" => line_tokens.push(Token::DiscardVar),
				"::" => line_tokens.push(Token::ScopeResolution),
				"=" => line_tokens.push(Token::Assignment),
				"!" => line_tokens.push(Token::ExclamationMark),
				"*" => line_tokens.push(Token::Star),
				"==" => line_tokens.push(Token::LogicalEqual),
				"!=" => line_tokens.push(Token::LogicalNotEqual),
				"<=" => line_tokens.push(Token::LessThanOrEqual),
				">=" => line_tokens.push(Token::MoreThanOrEqual),
				"<" => line_tokens.push(Token::LessThan),
				">" => line_tokens.push(Token::MoreThan),
				"+=" => line_tokens.push(Token::PlusEqual),
				"-=" => line_tokens.push(Token::MinusEqual),
				"++" => line_tokens.push(Token::PlusPlus),
				"pub" => line_tokens.push(Token::Pub),
				"true" => line_tokens.push(Token::LogicalTrue),
				"false" => line_tokens.push(Token::LogicalFalse),
				"raise" => line_tokens.push(Token::Raise),
				"await" => line_tokens.push(Token::Await),
				"default" => line_tokens.push(Token::DefaultKeyword),
				":" => line_tokens.push(Token::Colon),
				"let" => line_tokens.push(Token::Let),
				"" => (),
				 _	=> {
					char_token = string_token.chars().collect();
					if line_tokens.len() != 0 && line_tokens[line_tokens.len()-1] == Token::Tag { //@ tags first means identifer next
						match string_token.as_str(){
							"export" => line_tokens.push(Token::Export),
							"import" => line_tokens.push(Token::Import),
							"require" => line_tokens.push(Token::Require),
							"Event" => line_tokens.push(Token::Event),
							"EventHandler" => line_tokens.push(Token::EventHandler),
							"use" => line_tokens.push(Token::Use),
							_ => error_handler::error::error_reporter(error_handler::Error::new(
								String::from("Tag matching"), line_local.actual_line_num as i32, format!("Invalid tag: {}",string_token), line_text.iter().collect()
							))
						}
						let mut file_path = String::new();
							for token in line_split[i+1..].iter() {
								file_path = [file_path, token.clone()].concat();
							}
						line_tokens.push(Token::Identifier(file_path));
						
					} else if char_token[0] == '"' || char_token[0] == '\''{ //String
						char_token.remove(0);
						char_token.pop();
						line_tokens.push(Token::String(char_token.iter().collect()));
					} else if char_token[0].is_digit(10) { //Number
						if string_token.contains('.') { //Float
							match string_token.parse::<f32>() {
								Ok(val) => line_tokens.push(Token::Float(val)),
								Err(why) => error_handler::error::error_reporter(error_handler::Error::new (
									String::from("Float parsing"), line_local.actual_line_num as i32, format!("Invalid float {} Error: {}",string_token,why), line_text.iter().collect()
								))
							}
						} else { //Int
							match string_token.parse::<i32>() {
								Ok(val) => line_tokens.push(Token::Int(val)),
								Err(why) => error_handler::error::error_reporter(error_handler::Error::new (
									String::from("Int parsing"), 
									line_local.actual_line_num as i32, 
									format!("Invalid int {} Error: {}",string_token,why), 
									line_text.iter().collect()
								))
							}
						}
					} else { //Must be a variable
						line_tokens.push(Token::Identifier(string_token));
					}
					
				 }
			};
			i+=1;
		}
		//Everything should be done by now | Save shit in mutex that can't be poisoned
		line_local.line_token = line_tokens;
		let mut mutex_lines_data = lines_data.lock().unwrap();
		while mutex_lines_data.len() <= line_local.line_num { mutex_lines_data.push(Line::new())}
		mutex_lines_data[line_local.line_num] = line_local.clone();
		channel_thread_tx.send(0).unwrap();
	}).unwrap();
	return handle;
}