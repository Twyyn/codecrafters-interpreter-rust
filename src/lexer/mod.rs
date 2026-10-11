pub mod error;
mod token;

use self::{
    error::{LexError, SpannedError},
    token::{Literal, Token, TokenKind},
};

use std::{
    iter::{FusedIterator, Peekable},
    str::Chars,
};

use memchr::memchr2;

// -------------------------------------------------------------------------------------------------
// Lexer (Tokenizer) & Iterator
// -------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct Lexer<'src> {
    source: &'src str,
    chars: Peekable<Chars<'src>>,
    cursor: usize,
    index: u32,
    finished: bool,
}

impl<'src> Lexer<'src> {
    pub fn new(source: &'src str) -> Self {
        Self {
            source,
            chars: source.chars().peekable(),
            cursor: 0,
            index: 1,
            finished: false,
        }
    }

    pub fn tokenize(&mut self) -> (Vec<Token<'src>>, Vec<SpannedError<LexError>>) {
        let mut tokens = Vec::new();
        let mut errors = Vec::new();

        for result in self.by_ref() {
            match result {
                Ok(token) => tokens.push(token),
                Err(error) => errors.push(error),
            }
        }

        (tokens, errors)
    }

    fn scan_token(&mut self) -> Result<Token<'src>, SpannedError<LexError>> {
        self.skip_trivia();

        let offset = self.cursor;

        let Some(ch) = self.bump() else {
            return Ok(Token {
                kind: TokenKind::Eof,
                lexeme: "",
                literal: None,
                index: self.index,
            });
        };

        let kind = match ch {
            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,
            '{' => TokenKind::LeftBrace,
            '}' => TokenKind::RightBrace,
            ',' => TokenKind::Comma,
            '.' => TokenKind::Dot,
            ';' => TokenKind::Semicolon,
            '-' => TokenKind::Minus,
            '+' => TokenKind::Plus,
            '/' => TokenKind::Slash,
            '*' => TokenKind::Star,
            '!' => {
                if self.advance_if('=') {
                    TokenKind::BangEqual
                } else {
                    TokenKind::Bang
                }
            }

            '=' => {
                if self.advance_if('=') {
                    TokenKind::EqualEqual
                } else {
                    TokenKind::Equal
                }
            }

            '>' => {
                if self.advance_if('=') {
                    TokenKind::GreaterEqual
                } else {
                    TokenKind::Greater
                }
            }

            '<' => {
                if self.advance_if('=') {
                    TokenKind::LessEqual
                } else {
                    TokenKind::Less
                }
            }

            '"' => {
                return self.scan_string(offset);
            }

            '0'..='9' => {
                return self.scan_number(offset);
            }

            'a'..='z' | 'A'..='Z' | '_' => {
                return Ok(self.scan_identifier(offset));
            }

            _ => {
                return Err(SpannedError {
                    error: LexError::UnexpectedChar { ch },
                    index: self.index,
                });
            }
        };

        let lexeme = self.slice_to_cursor(offset);

        Ok(Token {
            kind,
            lexeme,
            literal: None,
            index: self.index,
        })
    }

    fn scan_identifier(&mut self, offset: usize) -> Token<'src> {
        self.advance_while(|ch| ch.is_ascii_alphanumeric() || ch == '_');

        let lexeme = self.slice_to_cursor(offset);

        let kind = match lexeme {
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "and" => TokenKind::And,
            "or" => TokenKind::Or,
            "nil" => TokenKind::Nil,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "for" => TokenKind::For,
            "while" => TokenKind::While,
            "return" => TokenKind::Return,
            "class" => TokenKind::Class,
            "fun" => TokenKind::Fun,
            "super" => TokenKind::Super,
            "this" => TokenKind::This,
            "var" => TokenKind::Var,
            "print" => TokenKind::Print,
            _ => TokenKind::Identifier,
        };

        Token {
            kind,
            lexeme,
            literal: None,
            index: self.index,
        }
    }

    fn scan_string(&mut self, offset: usize) -> Result<Token<'src>, SpannedError<LexError>> {
        let start_index = self.index;

        while let Some(ch) = self.bump() {
            match ch {
                '"' => {
                    let lexeme = self.slice_to_cursor(offset);
                    let value = &self.source[offset + 1..self.cursor - 1];

                    return Ok(Token {
                        kind: TokenKind::String,
                        lexeme,
                        literal: Some(Literal::String(value)),
                        index: start_index,
                    });
                }

                '\n' => break,
                _ => {}
            }
        }

        Err(SpannedError {
            error: LexError::UnterminatedString,
            index: start_index,
        })
    }

    fn scan_number(&mut self, offset: usize) -> Result<Token<'src>, SpannedError<LexError>> {
        self.advance_while(|ch| ch.is_ascii_digit());

        let remaining = self.remaining_bytes();

        if remaining.first() == Some(&b'.') && remaining.get(1).is_some_and(u8::is_ascii_digit) {
            self.bump();
            self.advance_while(|ch| ch.is_ascii_digit());
        }

        let lexeme = self.slice_to_cursor(offset);
        let value = lexeme.parse::<f64>().map_err(|error| SpannedError {
            error: LexError::InvalidNumber {
                text: lexeme.into(),
                error,
            },
            index: self.index,
        })?;

        if !value.is_finite() {
            return Err(SpannedError {
                error: LexError::NumberOverflow {
                    text: lexeme.into(),
                },
                index: self.index,
            });
        }

        Ok(Token {
            kind: TokenKind::Number,
            lexeme,
            literal: Some(Literal::Number(value)),
            index: self.index,
        })
    }

    fn skip_trivia(&mut self) {
        loop {
            self.advance_while(|ch| ch.is_ascii_whitespace());

            let bytes = self.remaining_bytes();

            if !bytes.starts_with(b"//") {
                break;
            }

            let comment_length = memchr2(b'\n', b'\r', bytes).unwrap_or(bytes.len());

            self.advance_bytes(comment_length);
        }
    }

    fn advance_while(&mut self, predicate: impl Fn(char) -> bool) {
        while self.peek().is_some_and(&predicate) {
            self.bump();
        }
    }

    fn advance_bytes(&mut self, n: usize) {
        let end = self.cursor + n;

        let remaining = &self.source[end..];

        self.chars = remaining.chars().peekable();
        self.cursor = end;
    }

    fn advance_if(&mut self, expected: char) -> bool {
        if self.peek() != Some(expected) {
            return false;
        }

        self.bump();
        true
    }

    fn bump(&mut self) -> Option<char> {
        let ch = self.chars.next()?;
        let offset = self.cursor;

        self.cursor += ch.len_utf8();

        match ch {
            '\r' => {
                self.index += 1;
            }

            '\n' => {
                let trailing = offset > 0 && self.source.as_bytes()[offset - 1] == b'\r';

                if !trailing {
                    self.index += 1;
                }
            }

            _ => {}
        }

        Some(ch)
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    fn slice_to_cursor(&self, offset: usize) -> &'src str {
        &self.source[offset..self.cursor]
    }

    fn remaining_bytes(&self) -> &'src [u8] {
        &self.source.as_bytes()[self.cursor..]
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, SpannedError<LexError>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        match self.scan_token() {
            Ok(token) if matches!(token.kind, TokenKind::Eof) => {
                self.finished = true;
                Some(Ok(token))
            }

            result => Some(result),
        }
    }
}

impl FusedIterator for Lexer<'_> {}
