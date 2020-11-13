use std::fmt;

pub mod tokenizer;

/// Info struct with relevant info about each line.
#[derive(Clone, Debug)]
pub struct Line {
    /// The text in the line
    pub line_text: String,
    /// The text split into string tokens
    pub line_split: Vec<String>,
    /// The parsed tokens in the line
    pub line_token: Vec<Token>,
    /// Scope data
    pub scope_indentation: i32,
    /// Parent scopes
    pub scope_id_chain: Vec<i32>,
    /// The instruction number
    pub line_num: usize,
    /// The relevant line number in the actual code (Used for errors)
    pub actual_line_num: usize,
    /// Start position in file
    pub line_char_start: usize,
    /// End position in file
    pub line_char_end: usize,
}

impl Line {
    /// Returns a struct with default values
    pub fn new() -> Line {
        return Line {
            line_text: String::new(),
            line_token: vec!(),
            line_split: vec!(),
            scope_indentation: 0,
            scope_id_chain: vec!(),
            line_num: 0,
            actual_line_num: 0,
            line_char_start: 0,
            line_char_end: 0,
        };
    }
}

/// All possible tokens and keywords, fully converted to after the tokenizer
#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    /// @
    Tag,
    /// [0-9]+
    Int(i32),
    /// [0-9]+.[0-9]+
    Float(f32),
    /// ""
    String(String),
    /// a-zA-Z[a-zA-Z_0-9]+
    Identifier(String),
    /// raise
    Raise,
    /// await
    Await,
    /// True, False
    Bool(bool),
    /// new
    New,
    /// (
    LBrace,
    /// )
    RBrace,
    /// {
    LCurlyBrace,
    /// }
    RCurlyBrace,
    /// [
    LSquareBrace,
    /// ]
    RSquareBrace,
    /// =
    Equal,
    /// !=
    LogicalNotEqual,
    /// &&
    LogicalAnd,
    /// ||
    LogicalOr,
    /// true
    LogicalTrue,
    /// false
    LogicalFalse,
    /// <
    LessThan,
    /// >
    GreaterThan,
    /// <=
    LessThanOrEqual,
    /// >=
    MoreThanOrEqual,
    /// +
    Plus,
    /// -
    Minus,
    /// +=
    PlusEqual,
    /// -=
    MinusEqual,
    /// ++
    PlusPlus,
    /// ->
    Arrow,
    /// |
    Pipe,
    /// !
    ExclamationMark,
    /// .
    Period,
    /// ,
    Comma,
    /// *
    Star,
    /// ::
    ScopeResolution,
    /// :
    Colon,
    /// =
    Assignment,
    /// ;
    SemiColon,
    /// _
    DiscardVar,
    /// pub
    Pub,
    /// export
    Export,
    /// enum
    Enum,
    /// while
    While,
    /// for
    For,
    /// loop
    Loop,
    /// if
    If,
    /// else
    Else,
    /// until
    Until,
    /// default
    DefaultKeyword,
    /// import
    Import,
    /// require
    Require,
    /// EventHandler
    EventHandler,
    /// Event
    Event,
    /// use
    Use,
    /// Package
    Package,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Token::Tag => format!("Tag"),
            Token::Int(n) => format!("Int {}", n),
            Token::Float(n) => format!("Float {}", n),
            Token::String(s) => format!("String {}", s),
            Token::Identifier(i) => format!("Identifier {}", i),
            Token::New => format!("Let"),
            Token::Raise => format!("Raise"),
            Token::Await => format!("Await"),
            Token::Bool(b) => format!("Bool {}", b),
            Token::LBrace => format!("LBrace"),
            Token::RBrace => format!("RBrace"),
            Token::RCurlyBrace => format!("RCurlyBrace"),
            Token::LCurlyBrace => format!("LCurlyBrace"),
            Token::LSquareBrace => format!("LSquareBrace"),
            Token::RSquareBrace => format!("RSquareBrace"),
            Token::Equal => format!("Equal"),
            Token::LogicalNotEqual => format!("LogicalNotEqual"),
            Token::LogicalAnd => format!("LogicalAnd"),
            Token::LogicalOr => format!("LogicalOr"),
            Token::LogicalTrue => format!("LogicalTrue"),
            Token::LogicalFalse => format!("LogicalFalse"),
            Token::LessThan => format!("LessThan"),
            Token::GreaterThan => format!("MoreThan"),
            Token::LessThanOrEqual => format!("LessThanOrEqual"),
            Token::MoreThanOrEqual => format!("MoreThanOrEqual"),
            Token::Plus => format!("Plus"),
            Token::Minus => format!("Minus"),
            Token::PlusEqual => format!("PlusEqual"),
            Token::MinusEqual => format!("MinusEqual"),
            Token::PlusPlus => format!("PlusPlus"),
            Token::Arrow => format!("Arrow"),
            Token::ExclamationMark => format!("ExclamationMark"),
            Token::Period => format!("Period"),
            Token::Comma => format!("Comma"),
            Token::Star => format!("Star"),
            Token::ScopeResolution => format!("ScopeResolution"),
            Token::Assignment => format!("Assignment"),
            Token::SemiColon => format!("SemiColon"),
            Token::DiscardVar => format!("DiscardVar"),
            Token::Pub => format!("Pub"),
            Token::Export => format!("Export"),
            Token::Import => format!("Import"),
            Token::Require => format!("Require"),
            Token::Enum => format!("Enum"),
            Token::While => format!("While"),
            Token::For => format!("For"),
            Token::Loop => format!("Loop"),
            Token::If => format!("If"),
            Token::Else => format!("Else"),
            Token::Until => format!("Until"),
            Token::Colon => format!("Colon"),
            Token::DefaultKeyword => format!("Default"),
            Token::EventHandler => format!("EventHandler"),
            Token::Event => format!("Event"),
            Token::Use => format!("Use"),
            Token::Package => format!("Package"),
            Token::Pipe => format!("|")
        })
    }
}


