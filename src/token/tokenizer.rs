use regex::Regex;

use super::Token;
use crate::error_handler::error::error_reporter;
use crate::error_handler::Error;
use std::process::exit;

/// Structure for storing a parsed file
#[derive(Debug, Clone)]
pub struct ParsedFile {
    /// The path of the file
    pub path: String,
    /// The original source code of the file, used for error reporting
    pub source: String,
    /// The tokenized source code
    pub tokens: Vec<ParsedToken>,
}

impl ParsedFile {
    /// Takes a string and converts it into a ParsedFile
    /// # Params
    /// - `source`: The original source code to be tokenized
    /// - `path`: The path of the source file
    /// # Returns
    /// The passed file
    pub fn new(source: &String, path: &String) -> ParsedFile {
        let chars: Vec<char> = source.chars().map(|x| match x { //Converts to Vec<char> and removes tabs and sometimes occurring \r newline
            '\t' => ' ',
            '\r' => ' ',
            _ => x
        }).collect::<Vec<char>>();

        let lines = source.split("\n").collect::<Vec<&str>>();

        // The current line that the program is parsing
        let mut line = 1;
        // Where the last token started from
        let mut start_char = 1;
        // Where the current token starts
        let mut tok_char = 1;

        // The accumulation of all characters throughout the tokenization process
        let mut token = String::new();

        // Vector of resulting parsed tokens
        let mut tokens = Vec::new();

        // Set whenever a string is being read
        let mut read_string = false;
        // Set whenever a multiline comment is encountered
        let mut ml_comment = false;

        // The index into the charter array to get the current token
        let mut i = 0;

        while i < source.len() {
            // Used for converting char to &str, has 0 use otherwise
            let mut tmp = [0; 4];

            // Get the current character
            let char = chars[i];


            if read_string {
                if char == '"' {
                    tokens.push(ParsedToken { token: Token::String(token.clone()), line, char: start_char });
                    token = String::new();
                    read_string = false;
                    start_char = tok_char;
                    i += 1;
                    continue;
                } else if char == '\n' {
                    line += 1;
                    tok_char = 0;
                }
                token += char.encode_utf8(&mut tmp);
            } else if ml_comment {
                println!("{}, {}", char, chars[i + 1]);
                if char == '*' && chars[i + 1] == '/' {
                    println!("Multiline finished");
                    ml_comment = false;
                }
            } else {
                match char {
                    '/' => {
                        i += 1;
                        match chars[i] {
                            '/' => {
                                if token != String::new() {
                                    tokens.push(ParsedToken {
                                        token: get_token(&token).unwrap_or_else(|| {
                                            error_reporter(Error::new(
                                                path.to_owned(),
                                                line,
                                                format!("Unidentified token: {}", token),
                                                lines[(line - 1) as usize].to_string()));
                                            panic!();
                                        }),
                                        line,
                                        char: start_char,
                                    });
                                }
                                while chars[i] != '\n' {
                                    i += 1;
                                }
                                line += 1;
                                token = String::new();
                            }
                            '*' => {
                                if token != String::new() {
                                    tokens.push(ParsedToken {
                                        token: get_token(&token).unwrap_or_else(|| {
                                            error_reporter(Error::new(
                                                path.to_owned(),
                                                line,
                                                format!("Unidentified token: {}", token),
                                                lines[(line - 1) as usize].to_string()));
                                            panic!();
                                        }),
                                        line,
                                        char: start_char,
                                    });
                                }
                                ml_comment = true;
                                token = String::new();
                            }
                            _ => {
                                error_reporter(Error::new(
                                    path.to_owned(),
                                    line,
                                    format!("Unidentified token: {}", token),
                                    lines[(line - 1) as usize].to_string()));
                            }
                        }
                    }
                    '\n' => {
                        line += 1;
                        if token != String::new() {
                            tokens.push(ParsedToken {
                                token: get_token(&token).unwrap_or_else(|| {
                                    error_reporter(Error::new(
                                        path.to_owned(),
                                        line,
                                        format!("Unidentified token: {}", token),
                                        lines[(line - 1) as usize].to_string()));
                                    panic!();
                                }),
                                line,
                                char: start_char,
                            });
                            token = String::new();
                        }
                        tok_char = 0;
                        start_char = 0;
                    }
                    '"' => {
                        if token != String::new() {
                            tokens.push(ParsedToken {
                                token: get_token(&token).unwrap_or_else(|| {
                                    error_reporter(Error::new(
                                        path.to_owned(),
                                        line,
                                        format!("Unidentified token: {}", token),
                                        lines[(line - 1) as usize].to_string()));
                                    panic!();
                                }),
                                line,
                                char: start_char,
                            });
                            token = String::new();
                        }
                        start_char = tok_char;
                        read_string = true;
                    }
                    ';' => {
                        if token != String::new() {
                            tokens.push(ParsedToken {
                                token: get_token(&token).unwrap_or_else(|| {
                                    error_reporter(Error::new(
                                        path.to_owned(),
                                        line,
                                        format!("Unidentified token: {}", token),
                                        lines[(line - 1) as usize].to_string()));
                                    panic!();
                                }),
                                line,
                                char: start_char,
                            });
                        }
                        tokens.push(ParsedToken { token: Token::SemiColon, line, char: tok_char });
                        start_char = tok_char;
                        token = String::new();
                    }
                    '*' => {
                        if token != String::new() {
                            tokens.push(ParsedToken {
                                token: get_token(&token).unwrap_or_else(|| {
                                    error_reporter(Error::new(
                                        path.to_owned(),
                                        line,
                                        format!("Unidentified token: {}", token),
                                        lines[(line - 1) as usize].to_string()));
                                    panic!();
                                }),
                                line,
                                char: start_char,
                            });
                        }
                        tokens.push(ParsedToken { token: Token::Star, line, char: tok_char });
                        start_char = tok_char + 1;
                        token = String::new();
                    }
                    '_' => {
                        if token != String::new() {
                            tokens.push(ParsedToken {
                                token: get_token(&token).unwrap_or_else(|| {
                                    error_reporter(Error::new(
                                        path.to_owned(),
                                        line,
                                        format!("Unidentified token: {}", token),
                                        lines[(line - 1) as usize].to_string()));
                                    panic!();
                                }),
                                line,
                                char: start_char,
                            });
                        }
                        tokens.push(ParsedToken { token: Token::DiscardVar, line, char: tok_char });
                        start_char = tok_char + 1;
                        token = String::new();
                    }
                    '(' => {
                        if token != String::new() {
                            tokens.push(ParsedToken {
                                token: get_token(&token).unwrap_or_else(|| {
                                    error_reporter(Error::new(
                                        path.to_owned(),
                                        line,
                                        format!("Unidentified token: {}", token),
                                        lines[(line - 1) as usize].to_string()));
                                    panic!();
                                }),
                                line,
                                char: start_char,
                            });
                        }
                        tokens.push(ParsedToken { token: Token::LBrace, line, char: tok_char });
                        start_char = tok_char + 1;
                        token = String::new();
                    }
                    ')' => {
                        if token != String::new() {
                            tokens.push(ParsedToken {
                                token: get_token(&token).unwrap_or_else(|| {
                                    error_reporter(Error::new(
                                        path.to_owned(),
                                        line,
                                        format!("Unidentified token: {}", token),
                                        lines[(line - 1) as usize].to_string()));
                                    panic!();
                                }),
                                line,
                                char: start_char,
                            });
                        }
                        tokens.push(ParsedToken { token: Token::RBrace, line, char: tok_char });
                        start_char = tok_char + 1;
                        token = String::new();
                    }
                    '{' => {
                        if token != String::new() {
                            tokens.push(ParsedToken {
                                token: get_token(&token).unwrap_or_else(|| {
                                    error_reporter(Error::new(
                                        path.to_owned(),
                                        line,
                                        format!("Unidentified token: {}", token),
                                        lines[(line - 1) as usize].to_string()));
                                    panic!();
                                }),
                                line,
                                char: start_char,
                            });
                        }
                        tokens.push(ParsedToken { token: Token::LCurlyBrace, line, char: tok_char });
                        start_char = tok_char + 1;
                        token = String::new();
                    }
                    '}' => {
                        if token != String::new() {
                            tokens.push(ParsedToken {
                                token: get_token(&token).unwrap_or_else(|| {
                                    error_reporter(Error::new(
                                        path.to_owned(),
                                        line,
                                        format!("Unidentified token: {}", token),
                                        lines[(line - 1) as usize].to_string()));
                                    panic!();
                                }),
                                line,
                                char: start_char,
                            });
                        }
                        tokens.push(ParsedToken { token: Token::RCurlyBrace, line, char: tok_char });
                        start_char = tok_char + 1;
                        token = String::new();
                    }
                    '[' => {
                        if token != String::new() {
                            tokens.push(ParsedToken {
                                token: get_token(&token).unwrap_or_else(|| {
                                    error_reporter(Error::new(
                                        path.to_owned(),
                                        line,
                                        format!("Unidentified token: {}", token),
                                        lines[(line - 1) as usize].to_string()));
                                    panic!();
                                }),
                                line,
                                char: start_char,
                            });
                        }
                        tokens.push(ParsedToken { token: Token::LSquareBrace, line, char: tok_char });
                        start_char = tok_char + 1;
                        token = String::new();
                    }
                    ']' => {
                        if token != String::new() {
                            tokens.push(ParsedToken {
                                token: get_token(&token).unwrap_or_else(|| {
                                    error_reporter(Error::new(
                                        path.to_owned(),
                                        line,
                                        format!("Unidentified token: {}", token),
                                        lines[(line - 1) as usize].to_string()));
                                    panic!();
                                }),
                                line,
                                char: start_char,
                            });
                        }
                        tokens.push(ParsedToken { token: Token::RSquareBrace, line, char: tok_char });
                        start_char = tok_char + 1;
                        token = String::new();
                    }
                    '.' => {
                        if token != String::new() {
                            tokens.push(ParsedToken {
                                token: get_token(&token).unwrap_or_else(|| {
                                    error_reporter(Error::new(
                                        path.to_owned(),
                                        line,
                                        format!("Unidentified token: {}", token),
                                        lines[(line - 1) as usize].to_string()));
                                    panic!();
                                }),
                                line,
                                char: start_char,
                            });
                        }
                        tokens.push(ParsedToken { token: Token::Period, line, char: tok_char });
                        start_char = tok_char + 1;
                        token = String::new();
                    }
                    ',' => {
                        if token != String::new() {
                            tokens.push(ParsedToken {
                                token: get_token(&token).unwrap_or_else(|| {
                                    error_reporter(Error::new(
                                        path.to_owned(),
                                        line,
                                        format!("Unidentified token: {}", token),
                                        lines[(line - 1) as usize].to_string()));
                                    panic!();
                                }),
                                line,
                                char: start_char,
                            });
                        }
                        tokens.push(ParsedToken { token: Token::Comma, line, char: tok_char });
                        start_char = tok_char + 1;
                        token = String::new();
                    }
                    ' ' => {
                        if token != String::new() {
                            tokens.push(ParsedToken {
                                token: get_token(&token).unwrap_or_else(|| {
                                    error_reporter(Error::new(
                                        path.to_owned(),
                                        line,
                                        format!("Unidentified token: {}", token),
                                        lines[(line - 1) as usize].to_string()));
                                    panic!();
                                }),
                                line,
                                char: start_char,
                            });
                        }
                        start_char = tok_char + 1;
                        token = String::new();
                    }
                    '=' => {
                        if token != String::new() {
                            tokens.push(ParsedToken {
                                token: get_token(&token).unwrap_or_else(|| {
                                    error_reporter(Error::new(
                                        path.to_owned(),
                                        line,
                                        format!("Unidentified token: {}", token),
                                        lines[(line - 1) as usize].to_string()));
                                    panic!();
                                }),
                                line,
                                char: start_char,
                            });
                        }
                        tokens.push(ParsedToken { token: Token::Equal, line, char: tok_char });
                        start_char = tok_char + 1;
                        token = String::new();
                    }
                    ':' => {
                        i += 1;
                        match chars[i] {
                            ':' => {
                                if token != String::new() {
                                    tokens.push(ParsedToken {
                                        token: get_token(&token).unwrap_or_else(|| {
                                            error_reporter(Error::new(
                                                path.to_owned(),
                                                line,
                                                format!("Unidentified token: {}", token),
                                                lines[(line - 1) as usize].to_string()));
                                            panic!();
                                        }),
                                        line,
                                        char: start_char,
                                    });
                                }
                                tokens.push(ParsedToken { token: Token::ScopeResolution, line, char: tok_char });
                                start_char = tok_char + 2;
                                token = String::new();
                            }
                            _ => {
                                if token != String::new() {
                                    tokens.push(ParsedToken {
                                        token: get_token(&token).unwrap_or_else(|| {
                                            error_reporter(Error::new(
                                                path.to_owned(),
                                                line,
                                                format!("Unidentified token: {}", token),
                                                lines[(line - 1) as usize].to_string()));
                                            panic!();
                                        }),
                                        line,
                                        char: start_char,
                                    });
                                }
                                tokens.push(ParsedToken { token: Token::Colon, line, char: tok_char });
                                start_char = tok_char + 1;
                                token = String::new();
                                i -= 1;
                            }
                        }
                    }
                    '!' => {
                        i += 1;
                        match chars[i] {
                            '=' => {
                                if token != String::new() {
                                    tokens.push(ParsedToken {
                                        token: get_token(&token).unwrap_or_else(|| {
                                            error_reporter(Error::new(
                                                path.to_owned(),
                                                line,
                                                format!("Unidentified token: {}", token),
                                                lines[(line - 1) as usize].to_string()));
                                            panic!();
                                        }),
                                        line,
                                        char: start_char,
                                    });
                                }
                                tokens.push(ParsedToken { token: Token::LogicalNotEqual, line, char: tok_char });
                                start_char = tok_char + 2;
                                token = String::new();
                            }
                            _ => {
                                if token != String::new() {
                                    tokens.push(ParsedToken {
                                        token: get_token(&token).unwrap_or_else(|| {
                                            error_reporter(Error::new(
                                                path.to_owned(),
                                                line,
                                                format!("Unidentified token: {}", token),
                                                lines[(line - 1) as usize].to_string()));
                                            panic!();
                                        }),
                                        line,
                                        char: start_char,
                                    });
                                }
                                tokens.push(ParsedToken { token: Token::ExclamationMark, line, char: tok_char });
                                start_char = tok_char + 1;
                                token = String::new();
                                i -= 1;
                            }
                        }
                    }
                    '&' => {
                        i += 1;
                        match chars[i] {
                            '&' => {
                                if token != String::new() {
                                    tokens.push(ParsedToken {
                                        token: get_token(&token).unwrap_or_else(|| {
                                            error_reporter(Error::new(
                                                path.to_owned(),
                                                line,
                                                format!("Unidentified token: {}", token),
                                                lines[(line - 1) as usize].to_string()));
                                            panic!();
                                        }),
                                        line,
                                        char: start_char,
                                    });
                                }
                                tokens.push(ParsedToken { token: Token::LogicalAnd, line, char: tok_char });
                                start_char = tok_char + 2;
                                token = String::new();
                            }
                            _ => {
                                if token != String::new() {
                                    tokens.push(ParsedToken {
                                        token: get_token(&token).unwrap_or_else(|| {
                                            error_reporter(Error::new(
                                                path.to_owned(),
                                                line,
                                                format!("Unidentified token: {}", token),
                                                lines[(line - 1) as usize].to_string()));
                                            panic!();
                                        }),
                                        line,
                                        char: start_char,
                                    });
                                }
                                tokens.push(ParsedToken { token: Token::Ampersand, line, char: tok_char });
                                start_char = tok_char + 1;
                                token = String::new();
                                i -= 1;
                            }
                        }
                    }
                    '|' => {
                        i += 1;
                        match chars[i] {
                            '|' => {
                                if token != String::new() {
                                    tokens.push(ParsedToken {
                                        token: get_token(&token).unwrap_or_else(|| {
                                            error_reporter(Error::new(
                                                path.to_owned(),
                                                line,
                                                format!("Unidentified token: {}", token),
                                                lines[(line - 1) as usize].to_string()));
                                            panic!();
                                        }),
                                        line,
                                        char: start_char,
                                    });
                                }
                                tokens.push(ParsedToken { token: Token::LogicalOr, line, char: tok_char });
                                start_char = tok_char + 2;
                                token = String::new();
                            }
                            _ => {
                                if token != String::new() {
                                    tokens.push(ParsedToken {
                                        token: get_token(&token).unwrap_or_else(|| {
                                            error_reporter(Error::new(
                                                path.to_owned(),
                                                line,
                                                format!("Unidentified token: {}", token),
                                                lines[(line - 1) as usize].to_string()));
                                            panic!();
                                        }),
                                        line,
                                        char: start_char,
                                    });
                                }
                                tokens.push(ParsedToken { token: Token::Pipe, line, char: tok_char });
                                start_char = tok_char + 1;
                                token = String::new();
                                i -= 1;
                            }
                        }
                    }
                    '+' => {
                        i += 1;
                        match chars[i] {
                            '=' => {
                                if token != String::new() {
                                    tokens.push(ParsedToken {
                                        token: get_token(&token).unwrap_or_else(|| {
                                            error_reporter(Error::new(
                                                path.to_owned(),
                                                line,
                                                format!("Unidentified token: {}", token),
                                                lines[(line - 1) as usize].to_string()));
                                            panic!();
                                        }),
                                        line,
                                        char: start_char,
                                    });
                                }
                                tokens.push(ParsedToken { token: Token::PlusEqual, line, char: tok_char });
                                start_char = tok_char + 2;
                                token = String::new();
                            }
                            '+' => {
                                if token != String::new() {
                                    tokens.push(ParsedToken {
                                        token: get_token(&token).unwrap_or_else(|| {
                                            error_reporter(Error::new(
                                                path.to_owned(),
                                                line,
                                                format!("Unidentified token: {}", token),
                                                lines[(line - 1) as usize].to_string()));
                                            panic!();
                                        }),
                                        line,
                                        char: start_char,
                                    });
                                }
                                tokens.push(ParsedToken { token: Token::PlusPlus, line, char: tok_char });
                                start_char = tok_char + 2;
                                token = String::new();
                            }
                            _ => {
                                if token != String::new() {
                                    tokens.push(ParsedToken {
                                        token: get_token(&token).unwrap_or_else(|| {
                                            error_reporter(Error::new(
                                                path.to_owned(),
                                                line,
                                                format!("Unidentified token: {}", token),
                                                lines[(line - 1) as usize].to_string()));
                                            panic!();
                                        }),
                                        line,
                                        char: start_char,
                                    });
                                }
                                tokens.push(ParsedToken { token: Token::Plus, line, char: tok_char });
                                start_char = tok_char + 2;
                                token = String::new();
                                i -= 1;
                            }
                        }
                    }
                    '-' => {
                        i += 1;
                        match chars[i] {
                            '=' => {
                                if token != String::new() {
                                    tokens.push(ParsedToken {
                                        token: get_token(&token).unwrap_or_else(|| {
                                            error_reporter(Error::new(
                                                path.to_owned(),
                                                line,
                                                format!("Unidentified token: {}", token),
                                                lines[(line - 1) as usize].to_string()));
                                            panic!();
                                        }),
                                        line,
                                        char: start_char,
                                    });
                                }
                                tokens.push(ParsedToken { token: Token::MinusEqual, line, char: tok_char });
                                start_char = tok_char + 2;
                                token = String::new();
                            }
                            '-' => {
                                if token != String::new() {
                                    tokens.push(ParsedToken {
                                        token: get_token(&token).unwrap_or_else(|| {
                                            error_reporter(Error::new(
                                                path.to_owned(),
                                                line,
                                                format!("Unidentified token: {}", token),
                                                lines[(line - 1) as usize].to_string()));
                                            panic!();
                                        }),
                                        line,
                                        char: start_char,
                                    });
                                }
                                tokens.push(ParsedToken { token: Token::MinusMinus, line, char: tok_char });
                                start_char = tok_char + 2;
                                token = String::new();
                            }
                            '>' => {
                                if token != String::new() {
                                    tokens.push(ParsedToken {
                                        token: get_token(&token).unwrap_or_else(|| {
                                            error_reporter(Error::new(
                                                path.to_owned(),
                                                line,
                                                format!("Unidentified token: {}", token),
                                                lines[(line - 1) as usize].to_string()));
                                            panic!();
                                        }),
                                        line,
                                        char: start_char,
                                    });
                                }
                                tokens.push(ParsedToken { token: Token::Arrow, line, char: tok_char });
                                start_char = tok_char + 2;
                                token = String::new();
                            }
                            _ => {
                                if token != String::new() {
                                    tokens.push(ParsedToken {
                                        token: get_token(&token).unwrap_or_else(|| {
                                            error_reporter(Error::new(
                                                path.to_owned(),
                                                line,
                                                format!("Unidentified token: {}", token),
                                                lines[(line - 1) as usize].to_string()));
                                            panic!();
                                        }),
                                        line,
                                        char: start_char,
                                    });
                                }
                                tokens.push(ParsedToken { token: Token::Minus, line, char: tok_char });
                                start_char = tok_char + 1;
                                token = String::new();
                                i -= 1;
                            }
                        }
                    }

                    '>' => {
                        i += 1;
                        match chars[i] {
                            '=' => {
                                if token != String::new() {
                                    tokens.push(ParsedToken {
                                        token: get_token(&token).unwrap_or_else(|| {
                                            error_reporter(Error::new(
                                                path.to_owned(),
                                                line,
                                                format!("Unidentified token: {}", token),
                                                lines[(line - 1) as usize].to_string()));
                                            panic!();
                                        }),
                                        line,
                                        char: start_char,
                                    });
                                }
                                tokens.push(ParsedToken { token: Token::GreaterThanOrEqual, line, char: tok_char });
                                start_char = tok_char + 1;
                                token = String::new();
                            }
                            _ => {
                                if token != String::new() {
                                    tokens.push(ParsedToken {
                                        token: get_token(&token).unwrap_or_else(|| {
                                            error_reporter(Error::new(
                                                path.to_owned(),
                                                line,
                                                format!("Unidentified token: {}", token),
                                                lines[(line - 1) as usize].to_string()));
                                            panic!();
                                        }),
                                        line,
                                        char: start_char,
                                    });
                                }
                                tokens.push(ParsedToken { token: Token::GreaterThan, line, char: tok_char });
                                start_char = tok_char + 1;
                                token = String::new();
                                i -= 1;
                            }
                        }
                    }
                    '<' => {
                        i += 1;
                        match chars[i] {
                            '=' => {
                                if token != String::new() {
                                    tokens.push(ParsedToken {
                                        token: get_token(&token).unwrap_or_else(|| {
                                            error_reporter(Error::new(
                                                path.to_owned(),
                                                line,
                                                format!("Unidentified token: {}", token),
                                                lines[(line - 1) as usize].to_string()));
                                            panic!();
                                        }),
                                        line,
                                        char: start_char,
                                    });
                                }
                                tokens.push(ParsedToken { token: Token::LessThanOrEqual, line, char: tok_char });
                                start_char = tok_char + 1;
                                token = String::new();
                            }
                            _ => {
                                if token != String::new() {
                                    tokens.push(ParsedToken {
                                        token: get_token(&token).unwrap_or_else(|| {
                                            error_reporter(Error::new(
                                                path.to_owned(),
                                                line,
                                                format!("Unidentified token: {}", token),
                                                lines[(line - 1) as usize].to_string()));
                                            panic!();
                                        }),
                                        line,
                                        char: start_char,
                                    });
                                }
                                tokens.push(ParsedToken { token: Token::LessThan, line, char: tok_char });
                                start_char = tok_char + 1;
                                token = String::new();
                                i -= 1;
                            }
                        }
                    }
                    _ => token += char.encode_utf8(&mut tmp),
                }
            }

            tok_char += 1;
            i += 1;
        }

        ParsedFile { path: path.to_owned(), source: source.to_owned(), tokens }
    }
    /// Gets the text on a given line
    /// # Params
    /// - `line`: The line number you want to get the text at
    pub fn resolve_line(&self, line: usize) -> Option<String> {
        Some(self.source.split("\n").collect::<Vec<&str>>().get(line)?.to_string())
    }
}

/// Structure for storing a single token
/// # Fields
/// - `token`: The token
/// - `line`: The line in source it was found on
/// - `char`: The position of the first character in source
#[derive(Clone, Debug)]
pub struct ParsedToken {
    pub token: Token,
    pub line: i32,
    pub char: i32,
}

/// Converts a `String` to a `Token`
/// # Params
/// - `source`: The string to be converted
/// # Returns
/// - `Some(Token)`: If a match can be found
/// - `None`: If it is not a valid token
fn get_token(source: &String) -> Option<Token> {
    if Regex::new("[a-zA-Z]([0-9a-zA-Z_]+)?").unwrap().is_match(source) {
        Some(Token::Identifier(source.to_owned()))
    } else if Regex::new("[0-9]+").unwrap().is_match(source) {
        Some(Token::Int(source.parse::<i32>().unwrap()))
    } else if Regex::new("[0-9].[0-9]+").unwrap().is_match(source) {
        Some(Token::Float(source.parse::<f32>().unwrap()))
    } else if Regex::new("@[a-zA-Z]+").unwrap().is_match(source) {
        Some(Token::Tag(source.replace("@", "")))
    } else if source == "pub" {
        Some(Token::Pub)
    } else if source == "while" {
        Some(Token::While)
    } else if source == "enum" {
        Some(Token::Enum)
    } else if source == "if" {
        Some(Token::If)
    } else if source == "else" {
        Some(Token::Else)
    } else if source == "true" {
        Some(Token::LogicalTrue)
    } else if source == "false" {
        Some(Token::LogicalFalse)
    } else if source == "new" {
        Some(Token::New)
    } else if source == "while" {
        Some(Token::While)
    } else if source == "loop" {
        Some(Token::Loop)
    } else if source == "for" {
        Some(Token::For)
    } else {
        None
    }
}
