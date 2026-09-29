use super::token::{Keyword, Token, TokenKind};
use crate::{DiagnosticReport, Error};
use crate::{
    error::LexError,
    span::{ByteIndex, LineIndex, Span},
};

type LexResult = Result<Token, DiagnosticReport>;

#[derive(Debug)]
pub struct Lexer<'src> {
    src: &'src str,
    scanner: Scanner<'src>,
    found_eof_token: bool,
}

impl<'src> Lexer<'src> {
    pub fn new(src: &'src str) -> Self {
        Self {
            src,
            scanner: Scanner::new(src),
            found_eof_token: false,
        }
    }

    fn next_token(&mut self) -> LexResult {
        self.skip_whitespace();
        self.scanner.mark_token_start();

        let Some(byte) = self.scanner.next() else {
            return Ok(self.token(TokenKind::Eof));
        };

        match byte {
            b'(' => Ok(self.token(TokenKind::LParen)),
            b')' => Ok(self.token(TokenKind::RParen)),
            b'{' => Ok(self.token(TokenKind::LBrace)),
            b'}' => Ok(self.token(TokenKind::RBrace)),
            b',' => Ok(self.token(TokenKind::Comma)),
            b'-' => Ok(self.token(TokenKind::Minus)),
            b'+' => Ok(self.token(TokenKind::Plus)),
            b';' => Ok(self.token(TokenKind::Semicolon)),
            b'*' => Ok(self.token(TokenKind::Star)),
            b'.' => Ok(self.token(TokenKind::Dot)),
            b'=' => {
                if self.scanner.eat_if(|byte| byte == b'=') {
                    Ok(self.token(TokenKind::EqualEqual))
                } else {
                    Ok(self.token(TokenKind::Equal))
                }
            }
            b'!' => {
                if self.scanner.eat_if(|byte| byte == b'=') {
                    Ok(self.token(TokenKind::BangEqual))
                } else {
                    Ok(self.token(TokenKind::Bang))
                }
            }
            b'<' => {
                if self.scanner.eat_if(|byte| byte == b'=') {
                    Ok(self.token(TokenKind::LessEqual))
                } else {
                    Ok(self.token(TokenKind::Less))
                }
            }
            b'>' => {
                if self.scanner.eat_if(|byte| byte == b'=') {
                    Ok(self.token(TokenKind::GreaterEqual))
                } else {
                    Ok(self.token(TokenKind::Greater))
                }
            }
            b'/' => Ok(self.token(TokenKind::Slash)),

            _ => {
                if byte == b'"' {
                    return self.string();
                }

                if byte.is_ascii_digit() {
                    return Ok(self.number());
                }

                if byte.is_ascii_alphanumeric() || byte == b'_' {
                    return Ok(self.identifier());
                }

                Err(DiagnosticReport {
                    error: Error::LexError(LexError::UnexpectedChar { ch: byte as char }),
                    span: self.span(),
                })
            }
        }
    }

    fn identifier(&mut self) -> Token {
        while self.scanner.eat_if(|b| b.is_ascii_alphanumeric()) {}

        let lexeme = &self.src[self.span()];
        let kind = lexeme
            .parse::<Keyword>()
            .map_or(TokenKind::Identifier, TokenKind::Keyword);

        self.token(kind)
    }

    fn number(&mut self) -> Token {
        while self.scanner.eat_if(|b| b.is_ascii_digit()) {}

        if self.scanner.eat(b'.') {
            while self.scanner.eat_if(|b| b.is_ascii_digit()) {}
        }

        self.token(TokenKind::Number)
    }

    fn string(&mut self) -> LexResult {
        if self.scanner.eat_until(b'"') {
            self.scanner.eat(b'"');

            return Ok(self.token(TokenKind::String));
        }

        Err(DiagnosticReport {
            error: Error::LexError(LexError::UnterminatedString),
            span: self.span(),
        })
    }

    fn skip_whitespace(&mut self) {
        loop {
            if self.scanner.starts_with(b"//") {
                self.skip_comment();
            }

            if self.scanner.eat_if(|b| b.is_ascii_whitespace()) {
                continue;
            }

            break;
        }
    }

    fn skip_comment(&mut self) {
        self.scanner.advance(2);
        self.scanner.eat_until(b'\n');
        self.scanner.eat(b'\n');
    }

    fn token(&self, kind: TokenKind) -> Token {
        Token::new(kind, self.span())
    }

    fn span(&self) -> Span {
        Span::new(self.scanner.token_start(), self.scanner.cursor())
    }
}

impl Iterator for Lexer<'_> {
    type Item = LexResult;

    fn next(&mut self) -> Option<Self::Item> {
        if self.found_eof_token {
            return None;
        }

        match self.next_token() {
            Ok(token) => {
                if token.kind() == TokenKind::Eof {
                    self.found_eof_token = true;
                }

                Some(Ok(token))
            }

            Err(err) => Some(Err(err)),
        }
    }
}

#[derive(Debug)]
struct Scanner<'src> {
    bytes: &'src [u8],
    cursor: ByteIndex,
    token_start: ByteIndex,
}

impl<'src> Scanner<'src> {
    pub fn new(src: &'src str) -> Self {
        Self {
            bytes: src.as_bytes(),
            cursor: ByteIndex::default(),
            token_start: ByteIndex::default(),
        }
    }

    fn remaining(&self) -> &[u8] {
        &self.bytes[self.cursor.as_usize()..]
    }

    fn peek(&self) -> Option<u8> {
        self.remaining().first().copied()
    }

    fn peek_next(&self) -> Option<u8> {
        self.remaining().get(1).copied()
    }

    fn starts_with(&self, bytes: &[u8]) -> bool {
        self.remaining().starts_with(bytes)
    }

    fn next(&mut self) -> Option<u8> {
        let byte = self.peek()?;
        self.advance(1);
        Some(byte)
    }

    fn advance(&mut self, count: usize) -> bool {
        if count > self.remaining().len() {
            return false;
        }

        self.cursor += ByteIndex::from(count);
        true
    }

    fn eat(&mut self, byte: u8) -> bool {
        self.eat_if(|current| current == byte)
    }

    fn eat_if<F>(&mut self, predicate: F) -> bool
    where
        F: FnOnce(u8) -> bool,
    {
        if self.peek().is_some_and(predicate) {
            self.advance(1);
            true
        } else {
            false
        }
    }

    fn eat_until(&mut self, byte: u8) -> bool {
        let index = {
            let remaining = self.remaining();
            memchr::memchr(byte, remaining).unwrap_or(remaining.len())
        };

        let found = index < self.remaining().len();

        self.advance(index);

        found
    }

    fn mark_token_start(&mut self) {
        self.token_start = self.cursor;
    }

    fn token_start(&self) -> ByteIndex {
        self.token_start
    }

    fn cursor(&self) -> ByteIndex {
        self.cursor
    }

    fn is_at_end(&self) -> bool {
        self.remaining().is_empty()
    }
}
