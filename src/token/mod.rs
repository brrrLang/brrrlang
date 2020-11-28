use std::fmt;

pub mod tokenizer;

/// All possible tokens and keywords, fully converted to after the tokenizer
#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    /// @[a-zA-Z]+
    Tag(String),
    /// [0-9]+
    Int(i32),
    /// [0-9]+.[0-9]+
    Float(f32),
    /// ""
    String(String),
    /// a-zA-Z[a-zA-Z_0-9]+
    Identifier(String),
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
    /// &
    Ampersand,
    /// <=
    LessThanOrEqual,
    /// >=
    GreaterThanOrEqual,
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
    /// --
    MinusMinus,
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
    /// ;
    SemiColon,
    /// _
    DiscardVar,
    /// pub
    Pub,
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
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Token::Tag(n) => format!("Tag {}", n),
            Token::Int(n) => format!("Int {}", n),
            Token::Float(n) => format!("Float {}", n),
            Token::String(s) => format!("String {}", s),
            Token::Identifier(i) => format!("Identifier {}", i),
            Token::New => format!("Let"),
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
            Token::GreaterThan => format!("GreaterThan"),
            Token::Ampersand => format!("Ampersand"),
            Token::LessThanOrEqual => format!("LessThanOrEqual"),
            Token::GreaterThanOrEqual => format!("GreaterThanOrEqual"),
            Token::Plus => format!("Plus"),
            Token::Minus => format!("Minus"),
            Token::PlusEqual => format!("PlusEqual"),
            Token::MinusEqual => format!("MinusEqual"),
            Token::PlusPlus => format!("PlusPlus"),
            Token::MinusMinus => format!("MinusMinus"),
            Token::Arrow => format!("Arrow"),
            Token::ExclamationMark => format!("ExclamationMark"),
            Token::Period => format!("Period"),
            Token::Comma => format!("Comma"),
            Token::Star => format!("Star"),
            Token::ScopeResolution => format!("ScopeResolution"),
            Token::SemiColon => format!("SemiColon"),
            Token::DiscardVar => format!("DiscardVar"),
            Token::Pub => format!("Pub"),
            Token::Enum => format!("Enum"),
            Token::While => format!("While"),
            Token::For => format!("For"),
            Token::Loop => format!("Loop"),
            Token::If => format!("If"),
            Token::Else => format!("Else"),
            Token::Colon => format!("Colon"),
            Token::Pipe => format!("|")
        })
    }
}
