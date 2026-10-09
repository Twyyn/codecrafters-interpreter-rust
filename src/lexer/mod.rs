mod token;

use self::token::{DiagnosticToken, Token, TokenKind};
use crate::{ByteIndex, Span};
use thiserror::Error;

#[derive(Debug)]
pub struct Lexer<'src> {
    bytes: &'src [u8],
    position: ByteIndex,
    line: usize,
}

impl<'src> Lexer<'src> {
    pub fn new(src: &'src str) -> Self {
        Self {
            bytes: src.as_bytes(),
            position: ByteIndex(0),
            line: 1,
        }
    }
    fn advance_token(&mut self) -> Result<Token, DiagnosticToken> {
        self.skip_trivia();

        let start = self.position;
        let line = self.line;

        match self.scan_token(start) {
            Ok(kind) => Ok(self.token(kind, start)),
            Err(error) => Err(self.error_token(&error, start, line)),
        }
    }

    fn scan_token(&mut self, start: ByteIndex) -> Result<TokenKind, LexError> {
        let Some(byte) = self.bump() else {
            return Ok(TokenKind::Eof);
        };

        let kind = match byte {
            b'(' => TokenKind::LeftParen,
            b')' => TokenKind::RightParen,
            b'{' => TokenKind::LeftBrace,
            b'}' => TokenKind::RightBrace,
            b',' => TokenKind::Comma,
            b'.' => TokenKind::Dot,
            b';' => TokenKind::Semicolon,
            b'-' => TokenKind::Minus,
            b'+' => TokenKind::Plus,
            b'/' => TokenKind::Slash,
            b'*' => TokenKind::Star,
            b'!' => {
                if self.match_next(b'=') {
                    TokenKind::BangEqual
                } else {
                    TokenKind::Bang
                }
            }
            b'=' => {
                if self.match_next(b'=') {
                    TokenKind::EqualEqual
                } else {
                    TokenKind::Equal
                }
            }

            b'>' => {
                if self.match_next(b'=') {
                    TokenKind::GreaterEqual
                } else {
                    TokenKind::Greater
                }
            }

            b'<' => {
                if self.match_next(b'=') {
                    TokenKind::LessEqual
                } else {
                    TokenKind::Less
                }
            }

            b'"' => self.string()?,
            b'0'..=b'9' => self.number(),
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => self.identifier(start),

            _ => {
                return Err(LexError::UnexpectedByte { byte });
            }
        };

        Ok(kind)
    }

    fn identifier(&mut self, start: ByteIndex) -> TokenKind {
        use token::{Keyword::*, Literal::Identifier};

        {
            self.consume_while(|byte| byte.is_ascii_alphanumeric() || byte == b'_');
        }

        match &self.bytes[start.into()..self.position.into()] {
            b"true" => TokenKind::Keyword(True),
            b"false" => TokenKind::Keyword(False),
            b"and" => TokenKind::Keyword(And),
            b"or" => TokenKind::Keyword(Or),
            b"nil" => TokenKind::Keyword(Nil),
            b"if" => TokenKind::Keyword(If),
            b"else" => TokenKind::Keyword(Else),
            b"for" => TokenKind::Keyword(For),
            b"while" => TokenKind::Keyword(While),
            b"return" => TokenKind::Keyword(Return),
            b"class" => TokenKind::Keyword(Class),
            b"fun" => TokenKind::Keyword(Fun),
            b"super" => TokenKind::Keyword(Super),
            b"this" => TokenKind::Keyword(This),
            b"var" => TokenKind::Keyword(Var),
            b"print" => TokenKind::Keyword(Print),
            _ => TokenKind::Literal(Identifier),
        }
    }

    fn string(&mut self) -> Result<TokenKind, LexError> {
        use token::Literal::String;

        while let Some(byte) = self.bump() {
            if byte == b'"' {
                return Ok(TokenKind::Literal(String));
            }
        }

        Err(LexError::UnterminatedString)
    }

    fn number(&mut self) -> TokenKind {
        use token::Literal::Number;

        {
            self.consume_while(|byte| byte.is_ascii_digit());
        }

        if self.peek() == Some(b'.') && self.remaining().get(1).is_some_and(u8::is_ascii_digit) {
            self.bump();
            {
                self.consume_while(|byte| byte.is_ascii_digit());
            }
        }

        TokenKind::Literal(Number)
    }

    fn skip_trivia(&mut self) {
        loop {
            {
                self.consume_while(|byte| byte.is_ascii_whitespace());
            }

            if self.remaining().starts_with(b"//") {
                self.bump();
                self.bump();

                {
                    self.consume_while(|byte| byte != b'\n');
                }

                continue;
            }
            break;
        }
    }

    fn consume_while(&mut self, predicate: impl Fn(u8) -> bool) {
        while self.peek().is_some_and(&predicate) {
            self.bump();
        }
    }

    fn remaining(&self) -> &[u8] {
        &self.bytes[self.position.into()..]
    }

    fn match_next(&mut self, expected: u8) -> bool {
        if self.peek() != Some(expected) {
            return false;
        }

        self.bump();
        true
    }

    fn bump(&mut self) -> Option<u8> {
        let byte = self.peek()?;
        self.position += 1;

        if byte == b'\n' {
            self.line += 1;
        }

        Some(byte)
    }

    fn peek(&self) -> Option<u8> {
        self.remaining().first().copied()
    }

    fn error_token(&self, error: &LexError, offset: ByteIndex, line: usize) -> DiagnosticToken {
        DiagnosticToken {
            message: error.to_string(),
            span: Span::new(offset, self.position),
            line,
        }
    }

    fn token(&self, kind: TokenKind, start: ByteIndex) -> Token {
        Token {
            kind,
            span: Span::new(start, self.position),
        }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Result<Token, DiagnosticToken>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.advance_token() {
            Ok(token) => {
                if matches!(token.kind, TokenKind::Eof) {
                    return None;
                }

                Some(Ok(token))
            }
            Err(error_token) => Some(Err(error_token)),
        }
    }
}

#[derive(Debug, Error)]
pub enum LexError {
    #[error("Unexpected byte: 0x{byte:02X}")]
    UnexpectedByte { byte: u8 },

    #[error("Unterminated string.")]
    UnterminatedString,
}
