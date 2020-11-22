use dyn_clone::private::process::exit;
use rayon::prelude::*;

use crate::error_handler::Error;
use crate::error_handler::error::error_reporter;
use crate::import_manager::ParsedFile;
use crate::token::{Line, Token};

#[derive(Debug)]
pub struct LexedFile {
    pub package: String,
    pub imports: Vec<String>,
    pub uses: Vec<String>,
    pub functions: Vec<Function>,
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub return_type: String,
    pub params: Vec<(String, String)>,
    pub code: Vec<(Token, String, i32)>,
}

pub fn lex(files: &Vec<ParsedFile>) -> Vec<LexedFile> {
    files.to_owned()
        .par_iter()
        .map(|file: &ParsedFile| {
            let mut package = "".to_string();
            let mut imports = vec!();
            let mut uses = vec!();
            for line in file.lines.iter() {
                println!("{:?}", line);
                for (a, token) in line.line_token.iter().enumerate() {
                    if token == &Token::Package {
                        if package.as_str() == "" {
                            package = match line.line_token.get(a + 1) {
                                None => {
                                    error_reporter(Error::new((&file.file_path).to_owned(),
                                                              line.line_num as i32,
                                                              "Can not find identifier for pkg declaration".parse().unwrap(),
                                                              (&line.line_text).to_owned()));
                                    exit(-1);
                                }
                                Some(token) => match token {
                                    Token::Identifier(val) => val.to_owned(),
                                    _ => {
                                        error_reporter(Error::new((&file.file_path).to_owned(),
                                                                  line.actual_line_num as i32,
                                                                  "Can not find identifier for pkg declaration".parse().unwrap(),
                                                                  (&line.line_text).to_owned()));
                                        exit(-1);
                                    }
                                }
                            }
                        } else {
                            error_reporter(Error::new((&file.file_path).to_owned(),
                                                      line.actual_line_num as i32,
                                                      "Multiple pkg declarations".parse().unwrap(),
                                                      (&line.line_text).to_owned()));
                            exit(-1);
                        }
                    } else if token == &Token::Import {
                        imports.push(
                            match line.line_token.get(a + 1) {
                                None => {
                                    error_reporter(Error::new((&line.line_text).to_owned(),
                                                              line.line_num as i32,
                                                              "Can not find identifier for import declaration".parse().unwrap(),
                                                              (&line.line_text).to_owned()));
                                    exit(-1);
                                }
                                Some(token) => match token {
                                    Token::Identifier(val) => val.to_owned(),
                                    _ => {
                                        error_reporter(Error::new((&file.file_path).to_owned(),
                                                                  line.line_num as i32,
                                                                  "Can not find identifier for import declaration".parse().unwrap(),
                                                                  (&line.line_text).to_owned()));
                                        exit(-1);
                                    }
                                }
                            });
                    } else if token == &Token::Use {
                        &uses.push(
                            match line.line_token.get(a + 1) {
                                None => {
                                    error_reporter(Error::new((&file.file_path).to_owned(),
                                                              line.line_num as i32,
                                                              "Can not find identifier for import declaration".parse().unwrap(),
                                                              (&line.line_text).to_owned()));
                                    exit(-1);
                                }
                                Some(token) => match token {
                                    Token::Identifier(val) => val.to_owned(),
                                    _ => {
                                        error_reporter(Error::new((&file.file_path).to_owned(),
                                                                  line.line_num as i32,
                                                                  "Can not find identifier for import declaration".parse().unwrap(),
                                                                  (&line.line_text).to_owned()));
                                        exit(-1);
                                    }
                                }
                            });
                    }
                }
            }

            let functions = vec!();

            LexedFile {
                functions,
                imports,
                package,
                uses,
            }
        })
        .collect::<Vec<LexedFile>>()
}