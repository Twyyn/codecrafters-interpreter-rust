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
    LeftBracket,
    RightBracket,

    EOF,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::LeftParen => "LEFT_PAREN",
            Self::RightParen => "RIGHT_PAREN",
            Self::LeftBracket => "LEFT_BRACKET",
            Self::RightBracket => "RIGHT_BRACKET",

            Self::EOF => "EOF",
        };

        write!(f, "{s}")
    }
}

#[derive(Debug)]
pub enum Literal {
    Number(f64),
    String(String),
    Nil,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{n}"),
            Self::String(s) => write!(f, "{s}"),
            Self::Nil => write!(f, "null"),
        }
    }
}
