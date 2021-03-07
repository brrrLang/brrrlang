use crate::config::Config;
use pest::Parser;

#[derive(Parser)]
#[grammar = "brrrLang.pest"]
pub struct BrrrLangParser;

pub fn compile(source: String, _config: Config) {
    let parsed = BrrrLangParser::parse(Rule::program, source.as_str());

    println!("{:#?}", parsed);
}
