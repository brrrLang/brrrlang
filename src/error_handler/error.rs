use ansi_term::Colour;
use std::process;

use crate::error_handler::*;

pub fn error_reporter(error: Error) {
	let mut error_text = format!("\n{} error", error.error_area );
	if error.line_num != -1 {
		error_text = format!("{}, line {}", error_text, error.line_num);
	}
	println!("{}{}",Colour::Red.bold().paint(error_text),Colour::Red.paint(format!(": {}",error.message)));
	if error.line_text != String::new() {
		println!("{}",Colour::Red.italic().paint(error.line_text));
	}
	process::exit(1);
}