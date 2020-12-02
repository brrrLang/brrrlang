use crate::token::tokenizer::ParsedFile;
use crate::token::Token;
use crate::error_handler::error::error_reporter;
use crate::error_handler::Error;

#[derive(Debug, Clone)]
pub struct LexedFile {
    pub path: String,
    pub source: String,
    pub uses: Vec<String>,
    pub imports: Vec<String>,
    pub pkg: String,
}

impl LexedFile {
    pub fn new(parsed_file: ParsedFile) -> LexedFile {
        let mut uses = vec!();
        let mut imports = vec!();
        let mut pkg = String::new();
        let tokens = parsed_file.tokens.clone();

        

        LexedFile {
            path: parsed_file.path,
            source: parsed_file.source,
            uses,
            pkg,
            imports,
        }
    }
}
