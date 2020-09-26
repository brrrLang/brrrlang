pub mod main;
use crate::token;



pub struct ParsedFile {
	pub lines: Vec<token::Line>,
	pub file_path: String,

}