use std::fmt;

#[derive(Copy, Clone)]
pub enum Tag {
	Export,
	Import,
	Require,
	Event,
	EventHandler,
}

impl fmt::Display for Tag {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", match self{
			Tag::Export => "Export",
			Tag::Import => "Import",
			Tag::Require => "Require",
			Tag::Event => "Event",
			Tag::EventHandler => "EventHandler"
		})
	}
}

#[derive(Clone)]
pub enum Token {
	Tag(Tag),           // @
	Number(i32),        // [0-9]+
	String(String),     // ""
	Identifier(String), // a-zA-Z[a-zA-Z_0-9]+
	Let(String),        // let
	Raise(String),      // raise
	Await(i32),         // await
	LBrace,             // (
	RBrace,             // )
	LCurlyBrace,       // {
	RCurlyBrace,       // }
	Period,             // .
	Comma,              // ,
	Star,               // *
	ScopeResolution,    // ::
	Assignment,         // =
	SemiColon,          // ;
	Pub,                // pub
	Require,            // require
}

impl fmt::Display for Token {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", match self {
			Token::Tag(t) 			=> format!("Tag {}", t),
			Token::Number(n) 			=> format!("Number {}", n),
			Token::String(s)		=> format!("String {}", s),
			Token::Identifier(i)	=> format!("Identifier {}", i),
			Token::Let(n)			=> format!("Let {}", i),
			Token::Raise(e)			=> format!("Raise {}", e),
			Token::Await(a)			=> format!("Await {}", a),
			Token::LBrace()					=> format!("LBrace"),
			Token::RBrace()					=> format!("RBrace"),
			Token::RCurlyBrace()			=> format!("RCurlyBrace"),
			Token::LCurlyBrace()			=> format!("LCurlyBrace"),
			Token::Period()					=> format!("Period"),
			Token::Comma()					=> format!("Comma"),
			Token::Star()					=> format!("Star"),
			Token::ScopeResolution()		=> format!("ScopeResolution"),
			Token::Assignment()				=> format!("Assignment"),
			Token::SemiColon()				=> format!("SemiColon"),
			Token::Pub()					=> format!("Pub"),
			Token::Require()				=> format!("Require")
		})
	}
}

fn tokenize(source: &String) -> Vec<Token> {
	let mut temp = String::new();
	let mut tokens: Vec<Token> = Vec::new();
	let chars = source.chars();
	let mut i = 0;

	while i < chars.len() {
		let char = chars[i];



		i += 1;
	}

tokens
}
