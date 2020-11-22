use std::io::prelude::*;
use std::process;

use ansi_term::Colour;

use crate::error_handler::*;

const STYLE: Colour = Colour::Red;

pub fn error_reporter(error: Error) {
    let out = std::io::stdout();
    let mut lock = out.lock();
    writeln!(
        &mut lock,
        "{}{}",
        STYLE.bold().paint(format!("\n{} error", error.error_area)),
        STYLE.paint(format!(": {}", error.message)
        ));
    if error.line_num != -1 {
        writeln!(
            &mut lock,
            "{}{}",
            STYLE.paint(format!(
                "Line {} {}",
                error.line_num,
                if error.line_text != String::new() {
                    String::from("--> ")
                } else {
                    String::new()
                }
            )),
            Colour::White.italic().paint(error.line_text)
        );
    } else if error.message != String::new() {
        writeln!(&mut lock, "{}{}", STYLE.paint("Line: "), Colour::White.italic().paint(error.line_text));
    }
    writeln!(&mut lock);
    drop(lock);

    process::exit(-1);
}