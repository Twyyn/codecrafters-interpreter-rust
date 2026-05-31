use crate::token::{KEYWORDS, Literal, Token, TokenKind};
use thiserror::Error;

#[derive(Debug)]
pub struct Lexer<'a> {
    cursor: LexerCursor<'a>,
    tokens: Vec<Token<'a>>,
    had_error: bool,
}

impl<'a> Lexer<'a> {
    pub const fn new(src: &'a str) -> Self {
        Self {
            cursor: LexerCursor::new(src),
            tokens: Vec::new(),
            had_error: false,
        }
    }

    pub fn scan_tokens(mut self) -> (Vec<Token<'a>>, bool) {
        while !self.cursor.is_at_end() {
            self.scan_token();
        }
        self.add_token(TokenKind::EOF);
        (self.tokens, self.had_error)
    }

    fn scan_token(&mut self) {
        self.cursor.reset_slice_offset();

        if let Some(c) = self.cursor.advance() {
            match c {
                '(' => self.add_token(TokenKind::LeftParen),
                ')' => self.add_token(TokenKind::RightParen),
                '{' => self.add_token(TokenKind::LeftBrace),
                '}' => self.add_token(TokenKind::RightBrace),

                ',' => self.add_token(TokenKind::Comma),
                '.' => self.add_token(TokenKind::Dot),
                '-' => self.add_token(TokenKind::Minus),
                '+' => self.add_token(TokenKind::Plus),
                ';' => self.add_token(TokenKind::Semicolon),
                '/' => {
                    if self.cursor.matches('/') {
                        self.comment();
                    } else {
                        self.add_token(TokenKind::Slash);
                    }
                }
                '*' => self.add_token(TokenKind::Star),

                '!' => {
                    let kind = if self.cursor.matches('=') {
                        TokenKind::BangEqual
                    } else {
                        TokenKind::Bang
                    };
                    self.add_token(kind);
                }
                '=' => {
                    let kind = if self.cursor.matches('=') {
                        TokenKind::EqualEqual
                    } else {
                        TokenKind::Equal
                    };
                    self.add_token(kind);
                }
                '<' => {
                    let kind = if self.cursor.matches('=') {
                        TokenKind::LessEqual
                    } else {
                        TokenKind::Less
                    };
                    self.add_token(kind);
                }
                '>' => {
                    let kind = if self.cursor.matches('=') {
                        TokenKind::GreaterEqual
                    } else {
                        TokenKind::Greater
                    };
                    self.add_token(kind);
                }

                c if c.is_ascii_digit() => {
                    if let Err(e) = self.number() {
                        eprintln!("{e}");
                    }
                }

                '"' => {
                    if let Err(e) = self.string() {
                        eprintln!("{e}");
                    }
                }

                c if c.is_ascii_alphanumeric() || c == '_' => self.identifier(),

                ' ' | '\r' | '\t' | '\n' => {}

                _ => {
                    self.had_error = true;
                    eprintln!(
                        "{}",
                        LexError::UnexpectedChar {
                            line: self.cursor.line,
                            c,
                        }
                    );
                }
            }
        }
    }

    fn add_token(&mut self, kind: TokenKind) {
        match kind {
            TokenKind::EOF => self
                .tokens
                .push(Token::new(kind, "", None, self.cursor.line)),

            _ => self.tokens.push(Token::new(
                kind,
                self.cursor.slice(),
                None,
                self.cursor.line,
            )),
        }
    }

    fn identifier(&mut self) {
        while self
            .cursor
            .peek()
            .is_some_and(|c| c.is_ascii_alphanumeric() || c == '_')
        {
            self.cursor.advance();
        }

        let lexeme = self.cursor.slice();

        if let Some(kind) = KEYWORDS.get(lexeme) {
            self.tokens
                .push(Token::new(kind.clone(), lexeme, None, self.cursor.line));
        } else {
            self.tokens.push(Token::new(
                TokenKind::Identifier,
                lexeme,
                None,
                self.cursor.line,
            ));
        }
    }

    fn number(&mut self) -> Result<(), LexError> {
        while self.cursor.peek().is_some_and(|c| c.is_ascii_digit()) {
            self.cursor.advance();
        }

        if self.cursor.peek() == Some('.')
            && self.cursor.peek_next().is_some_and(|c| c.is_ascii_digit())
        {
            self.cursor.advance();
            while self.cursor.peek().is_some_and(|c| c.is_ascii_digit()) {
                self.cursor.advance();
            }
        }

        let lexeme = self.cursor.slice();

        self.tokens.push(Token::new(
            TokenKind::Number,
            lexeme,
            Some(Literal::Number(lexeme.parse::<f64>()?)),
            self.cursor.line,
        ));

        Ok(())
    }

    fn string(&mut self) -> Result<(), LexError> {
        while self.cursor.peek().is_some_and(|c| c != '"') {
            self.cursor.advance();
        }

        if self.cursor.advance() != Some('"') {
            return Err(LexError::UnterminatedString {
                line: self.cursor.line,
            });
        }

        let lexeme = self.cursor.slice();

        self.tokens.push(Token::new(
            TokenKind::String,
            lexeme,
            Some(Literal::String(&lexeme[1..lexeme.len() - 1])),
            self.cursor.line,
        ));

        Ok(())
    }

    fn comment(&mut self) {
        while self.cursor.peek().is_some_and(|c| c != '\n') {
            self.cursor.advance();
        }
    }
}

#[derive(Debug)]
pub struct LexerCursor<'a> {
    src: &'a str,

    position: usize,
    slice_offset: usize,
    line: usize,
}

impl<'a> LexerCursor<'a> {
    pub const fn new(src: &'a str) -> Self {
        Self {
            src,
            position: 0,
            slice_offset: 0,
            line: 1,
        }
    }
    pub fn matches(&mut self, expected: char) -> bool {
        if self.peek() == Some(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn advance(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.position += c.len_utf8();

        if matches!(c, '\n') {
            self.line += 1;
        }

        Some(c)
    }

    pub const fn is_at_end(&self) -> bool {
        self.position >= self.src.len()
    }

    pub fn peek_next(&self) -> Option<char> {
        let mut lookahead = self.src[self.position..].char_indices();
        lookahead.next();
        lookahead.next().map(|(_, c)| c)
    }

    pub fn peek(&mut self) -> Option<char> {
        self.src[self.position..].chars().next()
    }

    pub const fn reset_slice_offset(&mut self) {
        self.slice_offset = self.position;
    }

    pub fn slice(&self) -> &'a str {
        &self.src[self.slice_offset..self.position]
    }
}

#[derive(Debug, Error)]
pub enum LexError {
    #[error("[line {line}] Error: Unexpected character: {c}")]
    UnexpectedChar { line: usize, c: char },

    #[error("[line {line}] Error: Unterminated string.")]
    UnterminatedString { line: usize },

    #[error("{0}")]
    FloatParse(#[from] std::num::ParseFloatError),
}
