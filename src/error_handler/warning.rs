use ansi_term::Colour;

use crate::error_handler::*;

const STYLE: Colour = Colour::Yellow;

pub fn warning_reporter(warning: Warning) {
	println!(
		"{}{}",
		STYLE.bold().paint(format!("\n{} warning", warning.error_area)),
		STYLE.paint(format!(": {}",warning.message)
	));
	if warning.line_num != -1 {
		println!(
			"{}{}",
			STYLE.paint(format!(
				"Line {} {}",
				warning.line_num,
				if warning.line_text != String::new() {
					String::from("")
				}
				else {
					String::new()
				}
			)),
			Colour::White.italic().paint(warning.line_text)
		);
	} else if warning.message != String::new() {
		println!("{}{}",STYLE.paint("Line: "),Colour::White.italic().paint(warning.line_text))
	}
	println!();
}