use crate::import_manager;
use crate::token::Token;

pub fn find_events(source: &Vec<import_manager::ParsedFile>) {
    for c in source.iter() {
        for r in c.lines.iter() {
            match r.line_token.iter().position(|d| d.to_owned().to_owned() == Token::Event) {
                Some(a) => println!("{}", r.line_token[a + 1]),
                None => {}
            }
        }
    }
}