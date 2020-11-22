use std::io::prelude::*;

use ansi_term::Colour;

use crate::error_handler::*;

const STYLE: Colour = Colour::Yellow;

pub fn warning_reporter(warning: Warning) {
    let out = std::io::stdout();
    let mut lock = out.lock();
    writeln!(
        &mut lock,
        "{}{}",
        STYLE.bold().paint(format!("\n{} warning", warning.error_area)),
        STYLE.paint(format!(": {}", warning.message)
        ));
    if warning.line_num != -1 {
        writeln!(
            &mut lock,
            "{}{}",
            STYLE.paint(format!(
                "Line {} {}",
                warning.line_num,
                if warning.line_text != String::new() {
                    String::from("")
                } else {
                    String::new()
                }
            )),
            Colour::White.italic().paint(warning.line_text)
        );
    } else if warning.message != String::new() {
        writeln!(&mut lock, "{}{}", STYLE.paint("Line: "), Colour::White.italic().paint(warning.line_text));
    }
    writeln!(&mut lock);
    drop(lock);
}