use super::token::{Keyword, Span, Token, TokenKind};
use crate::location::{ByteIndex, LineIndex, Location};
use std::str::CharIndices;
use thiserror::Error;

type LexResult<T> = Result<T, LexError>;

#[derive(Debug)]
pub struct Lexer<'src> {
    src: &'src str,
    chars: CharIndices<'src>,
    start: Location,
    current: Location,
    at_eof: bool,
}

impl<'src> Lexer<'src> {
    pub fn new(src: &'src str) -> Self {
        Self {
            src,
            chars: src.char_indices(),
            start: Location::default(),
            current: Location::default(),
            at_eof: false,
        }
    }

    fn next_token(&mut self) -> LexResult<Option<Token>> {
        let Some(ch) = self.bump() else {
            return Ok(Some(self.token(TokenKind::Eof)));
        };

        let kind = match ch {
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            '{' => TokenKind::LBrace,
            '}' => TokenKind::RBrace,
            ',' => TokenKind::Comma,
            '-' => TokenKind::Minus,
            '+' => TokenKind::Plus,
            ';' => TokenKind::Semicolon,
            '*' => TokenKind::Star,
            '.' => TokenKind::Dot,
            '=' if self.match_next(|ch| ch == '=') => TokenKind::EqualEqual,
            '=' => TokenKind::Equal,
            '!' if self.match_next(|ch| ch == '=') => TokenKind::BangEqual,
            '!' => TokenKind::Bang,
            '<' if self.match_next(|ch| ch == '=') => TokenKind::LessEqual,
            '<' => TokenKind::Less,
            '>' if self.match_next(|ch| ch == '=') => TokenKind::GreaterEqual,
            '>' => TokenKind::Greater,
            '"' => self.string()?,
            '/' => {
                if self.match_next(|ch| ch == '/') {
                    self.comment();
                    return Ok(None);
                }

                TokenKind::Slash
            }
            ch if ch.is_ascii_whitespace() => {
                self.whitespace();
                return Ok(None);
            }
            ch if ch.is_ascii_digit() => self.number(),
            ch if ch.is_alphanumeric() || ch == '_' => self.identifer(),
            _ => {
                return Err(LexError::UnexpectedChar {
                    line: self.current.line(),
                    ch,
                });
            }
        };

        Ok(Some(self.token(kind)))
    }

    fn identifer(&mut self) -> TokenKind {
        while matches!(self.peek(), Some(ch) if ch.is_alphanumeric()|| ch == '_') {
            self.bump();
        }

        self.span()
            .slice(self.src)
            .map_or(TokenKind::Identifier, |lexeme| {
                lexeme
                    .parse::<Keyword>()
                    .map_or(TokenKind::Identifier, TokenKind::Keyword)
            })
    }

    fn number(&mut self) -> TokenKind {
        while matches!(self.peek(), Some(ch) if ch.is_ascii_digit()) {
            self.bump();
        }

        if self.peek() == Some('.') && matches!(self.peek_next(), Some(ch) if ch.is_ascii_digit()) {
            self.bump();

            while matches!(self.peek(), Some(ch) if ch.is_ascii_digit()) {
                self.bump();
            }
        }

        TokenKind::Number
    }

    fn string(&mut self) -> LexResult<TokenKind> {
        while let Some(ch) = self.peek() {
            if ch == '"' {
                self.bump();
                return Ok(TokenKind::String);
            }

            self.bump();
        }

        Err(LexError::UnterminatedString {
            line: self.current.line(),
        })
    }

    fn whitespace(&mut self) -> Option<TokenKind> {
        while let Some(ch) = self.peek()
            && ch.is_ascii_whitespace()
        {
            self.bump();
        }

        None
    }

    fn comment(&mut self) -> Option<TokenKind> {
        while let Some(ch) = self.peek() {
            if ch == '\n' {
                break;
            }
            self.bump();
        }

        None
    }

    fn match_next<F>(&mut self, predicate: F) -> bool
    where
        F: FnOnce(char) -> bool,
    {
        match self.peek() {
            Some(ch) if predicate(ch) => {
                self.bump();
                true
            }
            _ => false,
        }
    }

    fn bump(&mut self) -> Option<char> {
        let (index, ch) = self.chars.next()?;
        self.current.advance(ByteIndex(index + ch.len_utf8()), ch);
        Some(ch)
    }

    fn peek_next(&self) -> Option<char> {
        let mut chars = self.chars.clone();

        chars.next();
        chars.next().map(|(_, ch)| ch)
    }

    fn peek(&self) -> Option<char> {
        self.chars.clone().next().map(|(_, ch)| ch)
    }

    fn token(&self, kind: TokenKind) -> Token {
        Token::new(kind, self.span())
    }

    fn span(&self) -> Span {
        Span::new(self.start.offset(), self.current.offset())
    }
}

impl Iterator for Lexer<'_> {
    type Item = LexResult<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.at_eof {
            return None;
        }

        loop {
            match self.next_token() {
                Ok(Some(token)) => {
                    if token.kind() == TokenKind::Eof {
                        self.at_eof = true;
                    }

                    return Some(Ok(token));
                }
                Ok(None) => {}

                Err(err) => return Some(Err(err)),
            }
        }
    }
}

#[derive(Debug, Error)]
pub enum LexError {
    #[error("[line {line}] Unexpected character: {ch}")]
    UnexpectedChar { line: LineIndex, ch: char },

    #[error("[line {line}] Unterminated string.")]
    UnterminatedString { line: LineIndex },
}
