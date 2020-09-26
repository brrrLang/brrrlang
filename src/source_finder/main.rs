use crate::token;

pub fn recursively_find_imports(all_parsed_tokens: &mut Vec<Vec<token::Line>>,cpu_thread_count: &usize, file_name: &String){
    let _tokens = token::tokenizer::parse_file(&file_name,&cpu_thread_count);
    all_parsed_tokens.push(_tokens);
}