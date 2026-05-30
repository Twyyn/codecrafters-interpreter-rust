use std::fmt;

#[derive(Debug)]
pub struct Token<'a> {
    kind: TokenKind,
    lexeme: &'a str,
    literal: Option<Literal>,
    #[allow(unused)]
    line: usize,
}

impl<'a> Token<'a> {
    pub const fn new(
        kind: TokenKind,
        lexeme: &'a str,
        literal: Option<Literal>,
        line: usize,
    ) -> Self {
        Self {
            kind,
            lexeme,
            literal,
            line,
        }
    }
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.literal {
            Some(literal) => write!(f, "{} {} {}", self.kind, self.lexeme, literal),
            None => write!(f, "{} {} null", self.kind, self.lexeme),
        }
    }
}

#[derive(Debug)]
pub enum TokenKind {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,

    Dot,
    Comma,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    Bang,
    Equal,
    Less,
    Greater,
    BangEqual,
    EqualEqual,
    LessEqual,
    GreaterEqual,

    String,
    Number,

    Identifier,

    EOF,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::LeftParen => "LEFT_PAREN",
            Self::RightParen => "RIGHT_PAREN",
            Self::LeftBrace => "LEFT_BRACE",
            Self::RightBrace => "RIGHT_BRACE",

            Self::Dot => "DOT",
            Self::Comma => "COMMA",
            Self::Minus => "MINUS",
            Self::Plus => "PLUS",
            Self::Semicolon => "SEMICOLON",
            Self::Slash => "SLASH",
            Self::Star => "STAR",

            Self::Bang => "BANG",
            Self::Equal => "EQUAL",
            Self::Less => "LESS",
            Self::Greater => "GREATER",
            Self::BangEqual => "BANG_EQUAL",
            Self::EqualEqual => "EQUAL_EQUAL",
            Self::LessEqual => "LESS_EQUAL",
            Self::GreaterEqual => "GREATER_EQUAL",

            Self::String => "STRING",
            Self::Number => "NUMBER",

            Self::Identifier => "IDENTIFIER",

            Self::EOF => "EOF",
        };

        write!(f, "{s}")
    }
}

#[derive(Debug)]
pub enum Literal {
    Number(f64),
    String(String),
}

#[derive(Debug)]
pub enum Keyword {
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(n) => {
                if n.fract() == 0.0 {
                    write!(f, "{n:.1}")
                } else {
                    write!(f, "{n}")
                }
            }
            Self::String(s) => write!(f, "{s}"),
        }
    }
}
