use std::fmt;
pub mod tokenizer;

#[derive(Clone, Debug)]
pub struct Line { //Holder obj with relevent info about each instructions
pub line_text: String,
	pub line_token: Vec<Token>,
	pub line_split: Vec<String>,
	pub scope_indentation: i32,
	pub scope_id_chain: Vec<i32>,
	pub line_num: usize,
	pub actual_line_num: usize,
	pub line_char_start: usize,
	pub line_char_end: usize,
}

impl Line {
	pub fn new() -> Line{
		return Line{
			line_text: String::new(),
			line_token: vec!(),
			line_split: vec!(),
			scope_indentation: 0,
			scope_id_chain: vec!(),
			line_num: 0,
			actual_line_num: 0,
			line_char_start: 0,
			line_char_end: 0,
		}
	}
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
	Tag,           		// @
	Number(i32),        // [0-9]+
	String(String),     // ""
	Identifier(String), // a-zA-Z[a-zA-Z_0-9]+
	Let(String),        // let
	Raise(String),      // raise
	Await(i32),         // await
	Bool(bool),			// True, False
	LBrace,             // (
	RBrace,             // )
	LCurlyBrace,        // {
	RCurlyBrace,        // }
	LSquareBrace,		// [
	RSquareBrace,		// ]
	Equal,				// =
	LogicalEqual,		// ==
	LogicalNotEqual,	// !=
	LogicalAnd,			// &&
	LogicalOr,			// ||
	LessThan,			// <
	MoreThan,			// >
	LessThanOrEqual,	// <=
	MoreThanOrEqual,	// >=
	Plus,				// +
	Minus,				// -
	PlusEqual,			// +=
	MinusEqual,			// -=
	PlusPlus,			// ++
	MatchArrow,			// =>
	ExclamationMark, 	// !
	Period,             // .
	Comma,              // ,
	Star,               // *
	ScopeResolution,    // ::
	Assignment,         // =
	SemiColon,          // ;
	DiscardVar,			// _
	Pub,                // pub
	Export,				// export
	Enum,				// enum
}

impl fmt::Display for Token {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", match self {
			Token::Tag			 		=> format!("Tag"),
			Token::Number(n) 			=> format!("Number {}", n),
			Token::String(s)			=> format!("String {}", s),
			Token::Identifier(i)		=> format!("Identifier {}", i),
			Token::Let(n)				=> format!("Let {}", n),
			Token::Raise(e)				=> format!("Raise {}", e),
			Token::Await(a)				=> format!("Await {}", a),
			Token::Bool(b)				=> format!("Bool {}", b),
			Token::LBrace				=> format!("LBrace"),
			Token::RBrace				=> format!("RBrace"),
			Token::RCurlyBrace			=> format!("RCurlyBrace"),
			Token::LCurlyBrace			=> format!("LCurlyBrace"),
			Token::LSquareBrace			=> format!("LSquareBrace"),
			Token::RSquareBrace			=> format!("RSquareBrace"),
			Token::Equal				=> format!("Equal"),
			Token::LogicalEqual			=> format!("LogicalEqual"),
			Token::LogicalNotEqual		=> format!("LogicalNotEqual"),
			Token::LogicalAnd			=> format!("LogicalAnd"),
			Token::LogicalOr			=> format!("LogicalOr"),
			Token::LessThan				=> format!("LessThan"),
			Token::MoreThan				=> format!("MoreThan"),
			Token::LessThanOrEqual		=> format!("LessThanOrEqual"),
			Token::MoreThanOrEqual		=> format!("MoreThanOrEqual"),
			Token::Plus					=> format!("Plus"),
			Token::Minus				=> format!("Minus"),
			Token::PlusEqual			=> format!("PlusEqual"),
			Token::MinusEqual			=> format!("MinusEqual"),
			Token::PlusPlus				=> format!("PlusPlus"),
			Token::MatchArrow			=> format!("MatchArrow"),
			Token::ExclamationMark		=> format!("ExclamationMark"),
			Token::Period				=> format!("Period"),
			Token::Comma				=> format!("Comma"),
			Token::Star					=> format!("Star"),
			Token::ScopeResolution		=> format!("ScopeResolution"),
			Token::Assignment			=> format!("Assignment"),
			Token::SemiColon			=> format!("SemiColon"),
			Token::DiscardVar			=> format!("DiscardVar"),
			Token::Pub					=> format!("Pub"),
			Token::Export				=> format!("Export"),
			Token::Enum					=> format!("Enum"),
		})
	}
}


