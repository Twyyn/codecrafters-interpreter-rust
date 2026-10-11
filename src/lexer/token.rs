use std::fmt;

// -------------------------------------------------------------------------------------------------
// Token
// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Token<'a> {
    kind: TokenKind,
    lexeme: &'a str,
    literal: Option<LiteralValue<'a>>,
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenKind, lexeme: &'a str, literal: Option<LiteralValue<'a>>) -> Self {
        Self {
            kind,
            lexeme,
            literal,
        }
    }

    pub fn kind(&self) -> TokenKind {
        self.kind
    }

    pub fn lexeme(&self) -> &'a str {
        self.lexeme
    }

    pub fn literal(&self) -> Option<LiteralValue<'a>> {
        self.literal
    }
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.literal {
            Some(literal) => write!(f, "{} {} {literal}", self.kind, self.lexeme),
            None => write!(f, "{} {} null", self.kind, self.lexeme),
        }
    }
}

// -------------------------------------------------------------------------------------------------
// Spanned (e.g., Location Of An Error)
// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Spanned<T> {
    inner: T,
    line: usize,
}

impl<T> Spanned<T>
where
    T: fmt::Display,
{
    pub fn new(inner: T, line: usize) -> Self {
        Self { inner, line }
    }

    pub fn inner(&self) -> &T {
        &self.inner
    }

    pub fn line(&self) -> usize {
        self.line
    }
}

impl<T: fmt::Display> fmt::Display for Spanned<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[line {}] Error: {}", self.line, self.inner)
    }
}

// -------------------------------------------------------------------------------------------------
// Token Kind
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LiteralValue<'a> {
    Number(f64),
    String(&'a str),
}

impl fmt::Display for LiteralValue<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(value) => {
                if value.fract() == 0.0 {
                    write!(f, "{value:.1}")
                } else {
                    write!(f, "{value}")
                }
            }
            Self::String(str) => write!(f, "{str}"),
        }
    }
}
