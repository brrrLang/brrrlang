use crate::token;
use std::process;

pub fn recursively_find_imports(all_parsed_tokens: &mut Vec<Vec<token::Line>>,cpu_thread_count: &usize, file_name: &String) {
    println!("Recursively finding imports");
    let _tokens = token::tokenizer::parse_file(&file_name,&cpu_thread_count);
    all_parsed_tokens.push(_tokens.clone());
    for a in _tokens.iter() {
        if a.line_token.len() >= 3 {
            match a.line_token[1] {
                token::Token::Import =>  {
                    let file_name_to_import = match &a.line_token[2] {
                        token::Token::Identifier(s) => s,
                        _ => {println!("Error in import tokenization"); process::exit(0x0100)},
                    };
                    recursively_find_imports(all_parsed_tokens,cpu_thread_count, file_name_to_import);
                    
                },
                _ => {}
            }
        }
    }
}