use crate::Span;
use std::fmt;

// -------------------------------------------------------------------------------------------------
// Token
// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at {:#?}", self.kind, self.span)
    }
}

#[derive(Debug, Clone)]
pub struct DiagnosticToken {
    pub message: String,
    pub span: Span,
    pub line: usize,
}

impl DiagnosticToken {
    pub fn new(message: impl Into<String>, span: Span, line: usize) -> Self {
        Self {
            message: message.into(),
            span,
            line,
        }
    }
}

impl fmt::Display for DiagnosticToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {} {:#?}", self.line, self.message, self.span)
    }
}
// -------------------------------------------------------------------------------------------------
// Token Kind
// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Literal {
    Number,
    String,
    Identifier,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Number => "NUMBER",
            Self::String => "STRING",
            Self::Identifier => "IDENTIFIER",
        };

        f.write_str(s)
    }
}

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
    Literal(Literal),

    // Keywords
    Keyword(Keyword),

    Eof,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
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
            Self::Literal(_) => "LITERAL",
            Self::Keyword(_) => "KEYWORD",
            Self::Eof => "EOF",
        };

        f.write_str(s)
    }
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::And => "AND",
            Self::Class => "CLASS",
            Self::Else => "ELSE",
            Self::False => "FALSE",
            Self::For => "FOR",
            Self::Fun => "FUN",
            Self::If => "IF",
            Self::Nil => "NIL",
            Self::Or => "OR",
            Self::Print => "PRINT",
            Self::Return => "RETURN",
            Self::Super => "SUPER",
            Self::This => "THIS",
            Self::True => "TRUE",
            Self::Var => "VAR",
            Self::While => "WHILE",
        };

        f.write_str(s)
    }
}

// -------------------------------------------------------------------------------------------------
// Token Formatter / Display
// -------------------------------------------------------------------------------------------------

// pub struct TokenDisplay<'token, 'src> {
//     token: &'token Token<'src>,
//     src: &'src str,
// }

// impl fmt::Display for TokenDisplay<'_, '_> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{} {} ", self.token.kind, self.token.lexeme(self.src))?;

//         match &self.token.kind {
//             TokenKind::String(value) => f.write_str(value),

//             TokenKind::Number(value) if value.fract() == 0.0 => {
//                 write!(f, "{value:.1}")
//             }
//             TokenKind::Number(value) => write!(f, "{value}"),

//             _ => f.write_str("null"),
//         }
//     }
// }
