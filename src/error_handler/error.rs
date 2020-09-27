use ansi_term::Colour;
use std::process;

use crate::error_handler::*;

const STYLE: Colour = Colour::Red;

pub fn error_reporter(error: Error) {
	println!(
		"{}{}",
		STYLE.bold().paint(format!("\n{} error", error.error_area)),
		STYLE.paint(format!(": {}",error.message)
	));
	if error.line_num != -1 {
		println!(
			"{}{}",
			STYLE.paint(format!(
				"Line {} {}",
				error.line_num,
				if error.line_text != String::new() {
					String::from("--> ")
				}
				else {
					String::new()
				}
			)),
			Colour::White.italic().paint(error.line_text)
		);
	}
	println!();
	process::exit(-1);
}