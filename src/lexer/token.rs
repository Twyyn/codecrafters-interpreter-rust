use crate::Span;
use std::fmt;

// -------------------------------------------------------------------------------------------------
// Token
// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Token {
    kind: TokenKind,
    length: u32,
}

impl Token {
    fn new(kind: TokenKind, length: u32) -> Token {
        Token { kind, length }
    }
}

pub struct SpannedToken {
    kind: TokenKind,
    span: Span,
}

impl SpannedToken {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }
}

// -------------------------------------------------------------------------------------------------
// Token Kinds
// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    // Punctuation
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Semicolon,

    // Operators
    Minus,
    Plus,
    Slash,
    Star,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Number,
    String,
    Identifier,

    // Keywords
    True,
    False,
    And,
    Or,
    Nil,
    If,
    Else,
    For,
    While,
    Return,
    Class,
    Fun,
    Super,
    This,
    Var,
    Print,

    Eof,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::LeftParen => "LEFT_PAREN",
            Self::RightParen => "RIGHT_PAREN",
            Self::LeftBrace => "LEFT_BRACE",
            Self::RightBrace => "RIGHT_BRACE",
            Self::Comma => "COMMA",
            Self::Dot => "DOT",
            Self::Minus => "MINUS",
            Self::Plus => "PLUS",
            Self::Semicolon => "SEMICOLON",
            Self::Slash => "SLASH",
            Self::Star => "STAR",
            Self::Bang => "BANG",
            Self::BangEqual => "BANG_EQUAL",
            Self::Equal => "EQUAL",
            Self::EqualEqual => "EQUAL_EQUAL",
            Self::Greater => "GREATER",
            Self::GreaterEqual => "GREATER_EQUAL",
            Self::Less => "LESS",
            Self::LessEqual => "LESS_EQUAL",
            Self::Number => "NUMBER",
            Self::String => "STRING",
            Self::Identifier => "IDENTIFIER",
            Self::True => "TRUE",
            Self::False => "FALSE",
            Self::And => "AND",
            Self::Or => "OR",
            Self::Nil => "NIL",
            Self::If => "IF",
            Self::Else => "ELSE",
            Self::For => "FOR",
            Self::While => "WHILE",
            Self::Return => "RETURN",
            Self::Class => "CLASS",
            Self::Fun => "FUN",
            Self::Super => "SUPER",
            Self::This => "THIS",
            Self::Var => "VAR",
            Self::Print => "PRINT",
            Self::Eof => "EOF",
        };

        f.write_str(s)
    }
}

// -------------------------------------------------------------------------------------------------
// Literal Values (i.e., the Concrete Values)
// -------------------------------------------------------------------------------------------------

// #[derive(Debug, Clone, Copy, PartialEq)]
// pub enum Literal<'a> {
//     Number(f64),
//     String(&'a str),
// }

// impl fmt::Display for Literal<'_> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Self::Number(value) => {
//                 if value.fract() == 0.0 {
//                     write!(f, "{value:.1}")
//                 } else {
//                     write!(f, "{value}")
//                 }
//             }
//             Self::String(str) => write!(f, "{str}"),
//         }
//     }
// }
