pub mod main;
use crate::token;

#[derive(Debug, Clone)]
pub struct ParsedFile {
	pub lines: Vec<token::Line>,
	pub file_path: String,

}