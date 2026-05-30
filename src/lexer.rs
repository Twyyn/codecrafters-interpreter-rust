use crate::token::{Literal, Token, TokenKind};
use thiserror::Error;

#[derive(Debug)]
pub struct Lexer<'a> {
    cursor: Cursor<'a>,
    tokens: Vec<Token<'a>>,
    current_position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            cursor: Cursor::new(src),
            tokens: Vec::new(),
            current_position: 0,
        }
    }

    pub fn scan_tokens(mut self) -> Result<Vec<Token<'a>>, Vec<Token<'a>>> {
        let mut had_error = false;

        loop {
            self.current_position = self.cursor.position();

            let Some(c) = self.cursor.advance() else {
                break;
            };

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
                        had_error = true;
                    }
                }

                '"' => {
                    if let Err(e) = self.string() {
                        eprintln!("{e}");
                        had_error = true;
                    }
                }

                c if c.is_ascii_alphanumeric() || matches!(c, '_') => self.identifier(),

                ' ' | '\r' | '\t' | '\n' => {}

                _ => {
                    let err = LexError::UnexpectedChar {
                        line: self.cursor.line,
                        c,
                    };
                    eprintln!("{err}");
                    had_error = true;
                }
            }
        }

        self.tokens
            .push(Token::new(TokenKind::EOF, "", None, self.cursor.line));

        if had_error {
            Err(self.tokens)
        } else {
            Ok(self.tokens)
        }
    }

    fn add_token(&mut self, kind: TokenKind) {
        self.tokens.push(Token::new(
            kind,
            self.cursor.slice(self.current_position),
            None,
            self.cursor.line,
        ));
    }

    fn identifier(&mut self) {
        while self
            .cursor
            .peek()
            .is_some_and(|c| c.is_ascii_alphanumeric() || matches!(c, '_'))
        {
            self.cursor.advance();
        }

        let mut lexeme = self.cursor.slice(self.current_position);
        if lexeme.starts_with('"') && lexeme.ends_with('"') {
            lexeme = &lexeme[1..lexeme.len() - 1];
        }

        self.tokens.push(Token::new(
            TokenKind::Identifier,
            lexeme,
            None,
            self.cursor.line,
        ));
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

        let lexeme = self.cursor.slice(self.current_position);

        self.tokens.push(Token::new(
            TokenKind::Number,
            lexeme,
            Some(Literal::Number(lexeme.parse::<f64>()?)),
            self.cursor.line,
        ));

        Ok(())
    }

    fn string(&mut self) -> Result<(), LexError> {
        while let Some(c) = self.cursor.peek() {
            if c == '"' {
                break;
            }
            self.cursor.advance();
        }

        if self.cursor.peek() != Some('"') {
            return Err(LexError::UnterminatedString {
                line: self.cursor.line,
            });
        }

        self.cursor.advance();

        let lexeme = self.cursor.slice(self.current_position);
        let value = &lexeme[1..lexeme.len() - 1];

        self.tokens.push(Token::new(
            TokenKind::Number,
            lexeme,
            Some(Literal::String(String::from(value))),
            self.cursor.line,
        ));

        Ok(())
    }

    fn comment(&mut self) {
        while let Some(c) = self.cursor.peek() {
            self.cursor.advance();
            if c == '\n' {
                break;
            }
        }
    }
}

#[derive(Debug)]
pub struct Cursor<'a> {
    src: &'a str,
    iter: std::iter::Peekable<std::str::CharIndices<'a>>,
    position: usize,
    line: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src,
            iter: src.char_indices().peekable(),
            position: 0,
            line: 1,
        }
    }

    pub fn advance(&mut self) -> Option<char> {
        let (offset, c) = self.iter.next()?;
        self.position = offset + c.len_utf8();

        if matches!(c, '\n') {
            self.line += 1;
        }

        Some(c)
    }

    pub fn matches(&mut self, expected: char) -> bool {
        if self.peek() == Some(expected) {
            self.advance();
            true
        } else {
            false
        }
    }
    pub fn peek_next(&mut self) -> Option<char> {
        let mut lookahead = self.src[self.position..].char_indices();
        lookahead.next();
        lookahead.next().map(|(_, c)| c)
    }

    pub fn peek(&mut self) -> Option<char> {
        self.iter.peek().map(|&(_, c)| c)
    }

    pub fn slice(&self, start: usize) -> &'a str {
        &self.src[start..self.position]
    }

    pub const fn position(&self) -> usize {
        self.position
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
